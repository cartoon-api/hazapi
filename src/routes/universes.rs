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
        .route("/{universe}", get(get_characters_by_universe))
}

async fn get_characters_by_universe(Extension(state): Extension<AppState>, Path(universe): Path<String>) -> Result<Json<Vec<Character>>, http::StatusCode> {
    let characters: Vec<Character> = sqlx::query_as!(
        Character,
        "SELECT name, data, specie, class, universe FROM characters WHERE universe = $1",
        universe
        )
        .fetch_all(&state.pool)
        .await
        .map_err(|_| http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(characters))
}
