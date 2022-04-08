use crate::{
    models::{jwt_claims::Claims, payloads::FindData, user},
    utils::prelude::*,
};
use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    Json,
};
use axum_auth::AuthBearer;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

type RouteResult<T, E> = Result<(StatusCode, T), (StatusCode, E)>;

pub async fn get_all_users(
    Extension(db): Extension<DatabaseConnection>,
) -> RouteResult<Json<Vec<user::Model>>, String> {
    let res = user::Entity::find().all(&db).await;

    match res {
        Ok(ids) => Ok((StatusCode::OK, Json(ids))),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

pub async fn get_user_by_id(
    Extension(db): Extension<DatabaseConnection>,
    Path(id): Path<i32>,
) -> RouteResult<Json<user::Model>, String> {
    let res = user::Entity::find_by_id(id).one(&db).await;

    match res {
        Ok(model) => match model {
            Some(model) => Ok((StatusCode::OK, Json(model))),
            None => Err((StatusCode::NOT_FOUND, "User not found".to_string())),
        },
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

pub async fn find_user(
    Extension(db): Extension<DatabaseConnection>,
    Query(payload): Query<FindData>,
) -> RouteResult<Json<Vec<user::Model>>, String> {
    let users = user::Entity::find()
        .filter(user::Column::Nickname.like(&payload.nickname))
        .all(&db)
        .await;

    match users {
        Ok(user) => Ok((StatusCode::OK, Json(user))),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

pub async fn get_self(
    Extension(db): Extension<DatabaseConnection>,
    Extension(config): Extension<Config>,
    AuthBearer(token): AuthBearer,
) -> RouteResult<Json<user::Model>, String> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = false;
    let claims_res = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(config.jwt_secret.as_ref()),
        &validation,
    );

    let claims = match claims_res {
        Ok(res) => res,
        Err(err) => return Err((StatusCode::UNAUTHORIZED, err.to_string())),
    }
    .claims;

    let user_res = user::Entity::find_by_id(claims.user_id).one(&db).await;

    let user = match user_res {
        Ok(res) => match res {
            Some(res) => res,
            None => return Err((StatusCode::NOT_FOUND, "User not found".to_string())),
        },
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Invalid token".to_string(),
            ))
        }
    };

    Ok((StatusCode::OK, Json(user)))
}
