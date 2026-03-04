use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use dashmap::DashMap;
use stratego::models::Game;

pub struct Storage {
    data_dir: PathBuf,
    /// In-memory cache: game ID → game state
    games: DashMap<String, Game>,
    /// In-memory index cache: "{type}_{hash}" → game ID
    index: DashMap<String, String>,
    /// Per-game mutex to serialize background disk writes
    write_locks: DashMap<String, Arc<tokio::sync::Mutex<()>>>,
}

impl Storage {
    pub fn new(data_dir: &Path) -> io::Result<Self> {
        let games_dir = data_dir.join("games");
        let index_dir = data_dir.join("index");
        fs::create_dir_all(&games_dir)?;
        fs::create_dir_all(&index_dir)?;
        Ok(Self {
            data_dir: data_dir.to_path_buf(),
            games: DashMap::new(),
            index: DashMap::new(),
            write_locks: DashMap::new(),
        })
    }

    fn get_write_lock(&self, game_id: &str) -> Arc<tokio::sync::Mutex<()>> {
        self.write_locks
            .entry(game_id.to_string())
            .or_insert_with(|| Arc::new(tokio::sync::Mutex::new(())))
            .clone()
    }

    /// Save a game: updates in-memory cache immediately, writes to disk in the background.
    pub async fn save_game(&self, game: &Game) -> io::Result<()> {
        // Update caches immediately (this is what all subsequent reads will see)
        self.games.insert(game.id.clone(), game.clone());
        self.index
            .insert(format!("red_{}", game.red_hash), game.id.clone());
        self.index
            .insert(format!("blue_{}", game.blue_hash), game.id.clone());
        self.index
            .insert(format!("join_{}", game.join_hash), game.id.clone());

        // Spawn background disk write
        let game = game.clone();
        let data_dir = self.data_dir.clone();
        let write_lock = self.get_write_lock(&game.id);

        tokio::spawn(async move {
            // Serialize the per-game write lock to prevent out-of-order writes
            let _guard = write_lock.lock().await;

            if let Err(e) = write_game_to_disk(&data_dir, &game).await {
                tracing::error!("Background write failed for game {}: {}", game.id, e);
            }
        });

        Ok(())
    }

    /// Load a game by its ID. Checks cache first, falls back to disk.
    pub async fn load_game_by_id(&self, id: &str) -> io::Result<Game> {
        if let Some(game) = self.games.get(id) {
            return Ok(game.clone());
        }

        let path = self.data_dir.join("games").join(format!("{}.json", id));
        let json = tokio::fs::read_to_string(&path).await?;
        let game: Game = serde_json::from_str(&json).map_err(io::Error::other)?;

        // Populate cache
        self.games.insert(game.id.clone(), game.clone());

        Ok(game)
    }

    /// Load a game by a hash (red_hash, blue_hash, or join_hash).
    pub async fn load_game_by_hash(&self, hash_type: &str, hash: &str) -> io::Result<Game> {
        let game_id = self.get_game_id_by_hash(hash_type, hash).await?;
        self.load_game_by_id(game_id.trim()).await
    }

    /// Look up a game by player_hash: try red first, then blue.
    /// Returns (game, side).
    pub async fn load_game_by_player_hash(&self, player_hash: &str) -> io::Result<(Game, i32)> {
        if let Ok(game) = self.load_game_by_hash("red", player_hash).await {
            return Ok((game, 0));
        }
        if let Ok(game) = self.load_game_by_hash("blue", player_hash).await {
            return Ok((game, 1));
        }
        Err(io::Error::new(io::ErrorKind::NotFound, "Game not found"))
    }

    /// Get the game_id for a given hash, without loading the full game.
    /// Checks index cache first, falls back to disk.
    pub async fn get_game_id_by_hash(&self, hash_type: &str, hash: &str) -> io::Result<String> {
        let key = format!("{}_{}", hash_type, hash);

        if let Some(game_id) = self.index.get(&key) {
            return Ok(game_id.clone());
        }

        let index_path = self.data_dir.join("index").join(&key);
        let game_id = tokio::fs::read_to_string(&index_path).await?;
        let game_id = game_id.trim().to_string();

        // Populate cache
        self.index.insert(key, game_id.clone());

        Ok(game_id)
    }
}

/// Write game state and index files to disk (called from background task).
async fn write_game_to_disk(data_dir: &Path, game: &Game) -> io::Result<()> {
    let game_path = data_dir.join("games").join(format!("{}.json", game.id));
    let tmp_path = data_dir
        .join("games")
        .join(format!("{}.json.tmp", game.id));

    let json = serde_json::to_string(game).map_err(io::Error::other)?;

    // Atomic write: write to tmp, then rename
    tokio::fs::write(&tmp_path, &json).await?;
    tokio::fs::rename(&tmp_path, &game_path).await?;

    // Write index files
    let index_dir = data_dir.join("index");
    tokio::fs::write(index_dir.join(format!("red_{}", game.red_hash)), &game.id).await?;
    tokio::fs::write(
        index_dir.join(format!("blue_{}", game.blue_hash)),
        &game.id,
    )
    .await?;
    tokio::fs::write(
        index_dir.join(format!("join_{}", game.join_hash)),
        &game.id,
    )
    .await?;

    Ok(())
}
