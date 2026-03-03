mod api;
mod build;
mod storage;
mod websocket;

use std::path::PathBuf;
use std::sync::Arc;

use axum::{
    http::{header, HeaderValue, Method},
    routing::{get, post},
    Router,
};
use clap::Parser;
use dashmap::DashMap;
use tokio::sync::{broadcast, Mutex};
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    services::{ServeDir, ServeFile},
};

pub struct AppState {
    pub game_locks: DashMap<String, Arc<tokio::sync::Mutex<()>>>,
    pub channels: DashMap<String, broadcast::Sender<String>>,
    pub pool: Mutex<Vec<api::PoolEntry>>,
    pub connected_sockets: DashMap<String, ()>,
    pub storage: storage::Storage,
}

impl AppState {
    pub fn get_game_lock(&self, game_id: &str) -> Arc<tokio::sync::Mutex<()>> {
        self.game_locks
            .entry(game_id.to_string())
            .or_insert_with(|| Arc::new(tokio::sync::Mutex::new(())))
            .clone()
    }
}

fn default_port() -> u16 {
    std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080)
}

#[derive(Parser)]
#[command(name = "stratego", about = "Stratego.io game server")]
struct Cli {
    /// Server port (defaults to PORT env var, then 8080)
    #[arg(short, long, default_value_t = default_port())]
    port: u16,

    /// Data directory for game persistence
    #[arg(short, long, default_value = "./data")]
    data_dir: PathBuf,

    /// Static files directory
    #[arg(short, long, default_value = "client/dist")]
    static_dir: PathBuf,

    /// Don't serve static files (API/WebSocket only)
    #[arg(long, env = "NO_STATIC")]
    no_static: bool,

    /// Force rebuild client assets before starting
    #[arg(long)]
    rebuild: bool,

    /// Build client assets and exit
    #[arg(long)]
    build_only: bool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    // Determine the project root (parent of client/)
    let project_root = std::env::current_dir().expect("Failed to get current directory");
    let client_dir = project_root.join("client");

    // Handle build logic
    if cli.build_only {
        tracing::info!("Building client assets...");
        if let Err(e) = build::build_client(&project_root, true) {
            tracing::error!("Build failed: {}", e);
            std::process::exit(1);
        }
        tracing::info!("Build complete.");
        return;
    }

    if !cli.no_static {
        // Skip auto-build if the static dir already has an index.html
        // (e.g. pre-built in Docker)
        let static_ready = cli.static_dir.join("index.html").exists();
        if cli.rebuild || (!static_ready && build::needs_build(&client_dir)) {
            let reason = if cli.rebuild {
                "rebuild requested"
            } else {
                "client/dist not found"
            };
            tracing::info!("Building client assets ({})...", reason);
            if let Err(e) = build::build_client(&project_root, true) {
                tracing::error!("Build failed: {}", e);
                std::process::exit(1);
            }
        }
    }

    let storage =
        storage::Storage::new(&cli.data_dir).expect("Failed to initialize storage");

    let state = Arc::new(AppState {
        game_locks: DashMap::new(),
        channels: DashMap::new(),
        pool: Mutex::new(Vec::new()),
        connected_sockets: DashMap::new(),
        storage,
    });

    let mut api_routes = Router::new()
        .route("/ws", get(websocket::ws_handler))
        .route("/api/create", post(api::create_handler))
        .route("/api/join", post(api::join_handler))
        .route("/api/move", post(api::move_handler))
        .route("/api/game", get(api::game_handler))
        .route("/api/pool/join", post(api::pool_join_handler))
        .with_state(state);

    // Add CORS layer when CORS_ORIGINS is set (for cross-origin deployments)
    let cors_origins = std::env::var("CORS_ORIGINS").unwrap_or_default();
    if !cors_origins.is_empty() {
        let origins: Vec<HeaderValue> = cors_origins
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        let cors = CorsLayer::new()
            .allow_origin(AllowOrigin::list(origins))
            .allow_methods([Method::GET, Method::POST])
            .allow_headers([header::CONTENT_TYPE]);
        api_routes = api_routes.layer(cors);
        tracing::info!("CORS enabled for: {}", cors_origins);
    }

    let app = if cli.no_static {
        api_routes
    } else {
        let static_dir = cli.static_dir.to_string_lossy().to_string();
        let index_path = format!("{}/index.html", static_dir);
        let fallback = ServeDir::new(&static_dir).fallback(ServeFile::new(&index_path));
        api_routes.fallback_service(fallback)
    };

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", cli.port))
        .await
        .expect("Failed to bind");

    tracing::info!("Stratego server listening on port {}", cli.port);
    axum::serve(listener, app).await.expect("Server error");
}
