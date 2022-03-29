use super::super::{
    models::{dto::LoginData, user},
    utils::prelude::*,
};
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    Json,
};
use sea_orm::{DatabaseConnection, EntityTrait};

type RouteResult<T> = Result<T, (StatusCode, String)>;

pub async fn get_all_identities(
    Extension(db): Extension<DatabaseConnection>,
) -> RouteResult<Json<Vec<user::Model>>> {
    let res = user::Entity::find().all(&db).await;

    match res {
        Ok(ids) => Ok(Json(ids)),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

pub async fn get_identity_by_id(
    Extension(db): Extension<DatabaseConnection>,
    Path(id): Path<i32>,
) -> RouteResult<Json<user::Model>> {
    let res = user::Entity::find_by_id(id).one(&db).await;

    match res {
        Ok(model) => match model {
            Some(model) => Ok(Json(model)),
            None => Err((StatusCode::NOT_FOUND, "Identity not found".to_string())),
        },
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

pub async fn login(
    Extension(config): Extension<Config>,
    payload: LoginData,
) -> RouteResult<String> {
    todo!();
}
