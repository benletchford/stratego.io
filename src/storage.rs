use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use stratego::models::Game;

pub struct Storage {
    data_dir: PathBuf,
}

impl Storage {
    pub fn new(data_dir: &Path) -> io::Result<Self> {
        let games_dir = data_dir.join("games");
        let index_dir = data_dir.join("index");
        fs::create_dir_all(&games_dir)?;
        fs::create_dir_all(&index_dir)?;
        Ok(Self {
            data_dir: data_dir.to_path_buf(),
        })
    }

    /// Save a game to disk with index files for hash lookups.
    pub async fn save_game(&self, game: &Game) -> io::Result<()> {
        let game_path = self.data_dir.join("games").join(format!("{}.json", game.id));
        let tmp_path = self
            .data_dir
            .join("games")
            .join(format!("{}.json.tmp", game.id));

        // Atomic write: write to tmp, then rename
        let json = serde_json::to_string_pretty(game)
            .map_err(io::Error::other)?;
        tokio::fs::write(&tmp_path, &json).await?;
        tokio::fs::rename(&tmp_path, &game_path).await?;

        // Write index files
        let index_dir = self.data_dir.join("index");
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

    /// Load a game by its ID.
    pub async fn load_game_by_id(&self, id: &str) -> io::Result<Game> {
        let path = self.data_dir.join("games").join(format!("{}.json", id));
        let json = tokio::fs::read_to_string(&path).await?;
        serde_json::from_str(&json).map_err(io::Error::other)
    }

    /// Load a game by a hash (red_hash, blue_hash, or join_hash).
    pub async fn load_game_by_hash(&self, hash_type: &str, hash: &str) -> io::Result<Game> {
        let index_path = self
            .data_dir
            .join("index")
            .join(format!("{}_{}", hash_type, hash));
        let game_id = tokio::fs::read_to_string(&index_path).await?;
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
    pub async fn get_game_id_by_hash(&self, hash_type: &str, hash: &str) -> io::Result<String> {
        let index_path = self
            .data_dir
            .join("index")
            .join(format!("{}_{}", hash_type, hash));
        let game_id = tokio::fs::read_to_string(&index_path).await?;
        Ok(game_id.trim().to_string())
    }
}
