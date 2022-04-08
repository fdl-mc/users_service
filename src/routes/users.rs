use crate::models::payloads::ChangePasswordData;
use crate::models::{payloads::FindData, responses::LoginResponse};

use crate::utils::generate_salt;
use crate::{
    models::{credential, jwt_claims::Claims, payloads::LoginData, user},
    utils::prelude::*,
};
use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    Json,
};
use axum_auth::AuthBearer;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

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

pub async fn login(
    Extension(db): Extension<DatabaseConnection>,
    Extension(config): Extension<Config>,
    Json(payload): Json<LoginData>,
) -> RouteResult<Json<LoginResponse>, String> {
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

    let claims = Claims {
        user_id: user.id,
        exp: 2147483647,
    };

    let jwt = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    );

    let jwt = match jwt {
        Ok(res) => res,
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    };

    Ok((StatusCode::OK, Json(LoginResponse { token: jwt })))
}

pub async fn change_password(
    Extension(config): Extension<Config>,
    Extension(db): Extension<DatabaseConnection>,
    AuthBearer(token): AuthBearer,
    Json(payload): Json<ChangePasswordData>,
) -> RouteResult<(), String> {
    // Validate and extract data from token
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

    // Find a credential model
    let credential_result = credential::Entity::find()
        .filter(credential::Column::UserId.eq(claims.user_id))
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

    // Update the password
    let mut credential: credential::ActiveModel = credential.into();

    let salt = generate_salt();
    let new_password = hash_password(payload.new_password, salt);

    credential.salt = Set(generate_salt());
    credential.password = Set(new_password);

    match credential.update(&db).await {
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
        _ => (),
    };

    Ok((StatusCode::NO_CONTENT, ()))
}
