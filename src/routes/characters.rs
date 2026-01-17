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
        .route("/hazbin_hotel", get(get_hazbin_hotel_characters))
        .route("/helluva_boss", get(get_helluva_boss_characters))
        .route("/{ch_name}", get(get_character_by_name))
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

async fn get_character_by_name(Extension(state): Extension<AppState>, Path(character_name): Path<String>) -> Result<Json<Character>, http::StatusCode> {
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

async fn get_hazbin_hotel_characters(Extension(state): Extension<AppState>) -> Result<Json<Vec<Character>>, http::StatusCode> {
    let characters: Vec<Character> = sqlx::query_as!(
        Character,
        "SELECT name, data, specie, class, universe FROM characters WHERE universe = 'hazbin_hotel' "
        )
        .fetch_all(&state.pool)
        .await
        .map_err(|_| http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(characters))
}


async fn get_helluva_boss_characters(Extension(state): Extension<AppState>) -> Result<Json<Vec<Character>>, http::StatusCode> {
    let characters: Vec<Character> = sqlx::query_as!(
        Character,
        "SELECT name, data, specie, class, universe FROM characters WHERE universe = 'helluva_boss' "
        )
        .fetch_all(&state.pool)
        .await
        .map_err(|_| http::StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(characters))
}

