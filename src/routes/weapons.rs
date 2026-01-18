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
use crate::routes::structures::Weapon;
use crate::app::AppState;

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_all_weapons))
        .route("/hazbin_hotel", get(get_hazbin_hotel_weapons))
        .route("/helluva_boss", get(get_helluva_boss_weapons))
        .route("/{weapon}", get(get_weapon_by_name))
}

async fn get_all_weapons(Extension(state): Extension<AppState>) -> Result<Json<Vec<Weapon>>, http::StatusCode> {
    let weapons: Vec<Weapon> = sqlx::query_as!(
        Weapon,
        "SELECT name, data, universe, owner FROM weapons"
        )
        .fetch_all(&state.pool)
        .await
        .map_err(|_| http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(weapons))
}

async fn get_weapon_by_name(Extension(state): Extension<AppState>, Path(weapon): Path<String>) -> Result<Json<Weapon>, http::StatusCode> {
    let wp: Option<Weapon> = sqlx::query_as!(
        Weapon,
        "SELECT name, data, universe, owner FROM weapons WHERE name = $1",
        weapon
        )
        .fetch_optional(&state.pool)
        .await
        .map_err(|_| http::StatusCode::INTERNAL_SERVER_ERROR)?;

    match wp {
        Some(w) => Ok(Json(w)),
        None => Err(http::StatusCode::NOT_FOUND)
    }
}

async fn get_hazbin_hotel_weapons(Extension(state): Extension<AppState>) -> Result<Json<Vec<Weapon>>, http::StatusCode> {
    let weapons: Vec<Weapon> = sqlx::query_as!(
        Weapon,
        "SELECT name, data, universe, owner FROM weapons WHERE universe = 'hazbin_hotel' "
        )
        .fetch_all(&state.pool)
        .await
        .map_err(|_| http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(weapons))
}

async fn get_helluva_boss_weapons(Extension(state): Extension<AppState>) -> Result<Json<Vec<Weapon>>, http::StatusCode> {
    let weapons: Vec<Weapon> = sqlx::query_as!(
        Weapon,
        "SELECT name, data, universe, owner FROM weapons WHERE universe = 'helluva_boss' "
        )
        .fetch_all(&state.pool)
        .await
        .map_err(|_| http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(weapons))
}

