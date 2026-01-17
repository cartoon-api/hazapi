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
        .route("/{specie}", get(get_character_by_specie))
}

async fn get_character_by_specie(Extension(state): Extension<AppState>, Path(specie): Path<String>) -> Result<Json<Vec<Character>>, http::StatusCode> {
    let characters: Vec<Character> = sqlx::query_as!(
        Character,
        "SELECT name, data, specie, class, universe FROM characters WHERE specie = $1",
        specie
        )
        .fetch_all(&state.pool)
        .await
        .map_err(|_| http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(characters))
}
