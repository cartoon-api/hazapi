// External USEs
use axum::{
    Router,
    Extension,
    Json,
    routing::get,
    extract::Path,
    http
};
use sqlx;

// Internal USEs
use crate::routes::structures::Character;
use crate::app::AppState;

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_all_characters))
        .route("/{character_name}", get(get_character))
}

async fn get_all_characters(Extension(state): Extension<AppState>) -> Result<Json<Vec<Character>>, http::StatusCode> {
    let characters: Vec<Character> = sqlx::query_as!(
        Character,
        "SELECT name, data, specie, class, universe FROM characters"
        )
        .fetch_all(&state.pool)
        .await
        .map_err(|_| http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(characters))
}

async fn get_character(Extension(state): Extension<AppState>, Path(character_name): Path<String>) -> Result<Json<Character>, http::StatusCode> {
    let character: Option<Character> = sqlx::query_as!(
        Character,
        "SELECT name, data, specie, class, universe FROM characters WHERE name = $1",
        character_name
        )
        .fetch_optional(&state.pool)
        .await
        .map_err(|_| http::StatusCode::INTERNAL_SERVER_ERROR)?;

    match character {
        Some(ch) => Ok(Json(ch)),
        None => Err(http::StatusCode::NOT_FOUND)
    }
}

