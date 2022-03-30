use crate::models::responses::LoginResponse;

use super::super::{
    models::{credential, jwt_claims::Claims, payloads::LoginData, user},
    utils::prelude::*,
};
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    Json,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

type RouteResult<T> = Result<T, (StatusCode, String)>;

pub async fn get_all_users(
    Extension(db): Extension<DatabaseConnection>,
) -> RouteResult<Json<Vec<user::Model>>> {
    let res = user::Entity::find().all(&db).await;

    match res {
        Ok(ids) => Ok(Json(ids)),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

pub async fn get_user_by_id(
    Extension(db): Extension<DatabaseConnection>,
    Path(id): Path<i32>,
) -> RouteResult<Json<user::Model>> {
    let res = user::Entity::find_by_id(id).one(&db).await;

    match res {
        Ok(model) => match model {
            Some(model) => Ok(Json(model)),
            None => Err((StatusCode::NOT_FOUND, "User not found".to_string())),
        },
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

pub async fn login(
    Extension(db): Extension<DatabaseConnection>,
    Extension(config): Extension<Config>,
    Json(payload): Json<LoginData>,
) -> RouteResult<Json<LoginResponse>> {
    let user_result = user::Entity::find()
        .filter(user::Column::Nickname.eq(payload.username))
        .one(&db)
        .await;

    let user = match user_result {
        Ok(res) => match res {
            Some(user) => user,
            None => return Err((StatusCode::NOT_FOUND, "User not found".to_string())),
        },
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    };

    let credential_result = credential::Entity::find()
        .filter(credential::Column::UserId.eq(user.id))
        .one(&db)
        .await;

    let credential = match credential_result {
        Ok(res) => match res {
            Some(credential) => credential,
            None => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Credential not found".to_string(),
                ))
            }
        },
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    };

    if hash_password(payload.password, credential.salt.clone()) != credential.password {
        return Err((StatusCode::FORBIDDEN, "Incorrect password".to_string()));
    }

    let claims = Claims { user_id: user.id };

    let jwt = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    );

    let jwt = match jwt {
        Ok(res) => res,
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    };

    Ok(Json(LoginResponse { token: jwt }))
}
