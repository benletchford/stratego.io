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

/// Join the matchmaking pool. Returns JSON with either:
/// - `{"matched": true, "player_hash": "..."}` if immediately matched
/// - `{"matched": false, "poll_id": "..."}` if queued (poll for match)
pub async fn join_pool(board: &str) -> Result<serde_json::Value, String> {
    let body = format!(
        "board={}",
        js_sys::encode_uri_component(board),
    );
    let resp = Request::post(&format!("{}/api/pool/join", config::api_base_url()))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let resp = check_response(resp).await?;
    resp.json().await.map_err(|e| e.to_string())
}

/// Check if a pool entry has been matched. Returns `Some(player_hash)` if matched.
pub async fn poll_pool(poll_id: &str) -> Result<Option<String>, String> {
    let url = format!(
        "{}/api/pool/status?poll_id={}",
        config::api_base_url(),
        js_sys::encode_uri_component(poll_id)
    );
    let resp = Request::get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let resp = check_response(resp).await?;
    let data: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;

    if data.get("matched").and_then(|v| v.as_bool()) == Some(true) {
        Ok(data
            .get("player_hash")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()))
    } else {
        Ok(None)
    }
}
