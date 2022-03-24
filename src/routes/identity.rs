use super::super::models::identity;
use axum::{extract::Extension, http::StatusCode, Json};
use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn get_all_identities(
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<Vec<identity::Model>>, (StatusCode, Json<String>)> {
    match identity::Entity::find().all(&db).await {
        Ok(ids) => Ok(Json(ids)),
        Err(error) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error.to_string()))),
    }
}
