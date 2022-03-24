use super::super::models::identity;
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    Json,
};
use sea_orm::{DatabaseConnection, EntityTrait};

pub async fn get_all_identities(
    Extension(db): Extension<DatabaseConnection>,
) -> Result<Json<Vec<identity::Model>>, (StatusCode, String)> {
    let res = identity::Entity::find().all(&db).await;

    match res {
        Ok(ids) => Ok(Json(ids)),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

pub async fn get_identity_by_id(
    Extension(db): Extension<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<Json<identity::Model>, (StatusCode, String)> {
    let res = identity::Entity::find_by_id(id).one(&db).await;

    match res {
        Ok(model) => match model {
            Some(model) => Ok(Json(model)),
            None => Err((StatusCode::NOT_FOUND, "Identity not found".to_string())),
        },
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}
