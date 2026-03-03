use gloo_net::http::{Request, Response};
use stratego::models::Position;

use crate::config;

/// Response shape from the server for game-related endpoints.
pub type GameResponse = serde_json::Value;

/// Check response status and return the body text as an error for non-success responses.
async fn check_response(resp: Response) -> Result<Response, String> {
    if resp.ok() {
        Ok(resp)
    } else {
        let text = resp.text().await.unwrap_or_default();
        Err(if text.is_empty() {
            format!("Request failed ({})", resp.status())
        } else {
            text
        })
    }
}

pub async fn create_game(board: &str) -> Result<GameResponse, String> {
    let body = format!("board={}", js_sys::encode_uri_component(board));
    let resp = Request::post(&format!("{}/api/create", config::api_base_url()))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    resp.json().await.map_err(|e| e.to_string())
}

pub async fn join_game(join_hash: &str, board: &str) -> Result<GameResponse, String> {
    let body = format!(
        "join_hash={}&board={}",
        js_sys::encode_uri_component(join_hash),
        js_sys::encode_uri_component(board)
    );
    let resp = Request::post(&format!("{}/api/join", config::api_base_url()))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    resp.json().await.map_err(|e| e.to_string())
}

pub async fn make_move(
    player_hash: &str,
    side: i32,
    from: &Position,
    to: &Position,
) -> Result<GameResponse, String> {
    let from_json = serde_json::to_string(from).unwrap();
    let to_json = serde_json::to_string(to).unwrap();
    let body = format!(
        "player_hash={}&side={}&from={}&to={}",
        js_sys::encode_uri_component(player_hash),
        side,
        js_sys::encode_uri_component(&from_json),
        js_sys::encode_uri_component(&to_json)
    );
    let resp = Request::post(&format!("{}/api/move", config::api_base_url()))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    resp.json().await.map_err(|e| e.to_string())
}

pub async fn get_game(player_hash: &str) -> Result<GameResponse, String> {
    let url = format!("{}/api/game?player_hash={}", config::api_base_url(), player_hash);
    let resp = Request::get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let resp = check_response(resp).await?;
    resp.json().await.map_err(|e| e.to_string())
}

/// Returns `Some(player_hash)` if immediately matched, `None` if queued in pool.
pub async fn join_pool(board: &str, socket_id: &str) -> Result<Option<String>, String> {
    let body = format!(
        "board={}&socket_id={}",
        js_sys::encode_uri_component(board),
        js_sys::encode_uri_component(socket_id)
    );
    let resp: serde_json::Value = Request::post(&format!("{}/api/pool/join", config::api_base_url()))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    if resp.get("matched").and_then(|v| v.as_bool()) == Some(true) {
        let hash = resp
            .get("player_hash")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        Ok(Some(hash))
    } else {
        Ok(None)
    }
}
