use super::super::models::identity;
use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    Json,
};
use sea_orm::{DatabaseConnection, EntityTrait};

type RouteResult<T> = Result<T, (StatusCode, String)>;

pub async fn get_all_identities(
    Extension(db): Extension<DatabaseConnection>,
) -> RouteResult<Json<Vec<identity::Model>>> {
    let res = identity::Entity::find().all(&db).await;

    match res {
        Ok(ids) => Ok(Json(ids)),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

pub async fn get_identity_by_id(
    Extension(db): Extension<DatabaseConnection>,
    Path(id): Path<i32>,
) -> RouteResult<Json<identity::Model>> {
    let res = identity::Entity::find_by_id(id).one(&db).await;

    match res {
        Ok(model) => match model {
            Some(model) => Ok(Json(model)),
            None => Err((StatusCode::NOT_FOUND, "Identity not found".to_string())),
        },
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

pub async fn auth_callback(
    Extension(db): Extension<DatabaseConnection>,
    Query(code): Query<String>,
) -> RouteResult<Json<identity::Model>> {
    todo!()
}
