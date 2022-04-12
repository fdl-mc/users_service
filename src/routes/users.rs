use crate::{
    models::{
        config::Config,
        credential,
        jwt_claims::Claims,
        payloads::{CreateUserPayload, FindPayload},
        user,
    },
    utils,
};
use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    Json,
};
use axum_auth::AuthBearer;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

type RouteResult<T, E> = Result<(StatusCode, T), (StatusCode, E)>;

/// Fetch all users.
pub async fn get_all_users(
    Extension(db): Extension<DatabaseConnection>,
) -> RouteResult<Json<Vec<user::Model>>, String> {
    let res = user::Entity::find().all(&db).await;

    match res {
        Ok(ids) => Ok((StatusCode::OK, Json(ids))),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}

/// Fetch a user by ID.
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

/// Find a user by nickname (or something else).
pub async fn find_user(
    Extension(db): Extension<DatabaseConnection>,
    Query(payload): Query<FindPayload>,
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

/// Fetch user by access token
pub async fn get_self(
    Extension(db): Extension<DatabaseConnection>,
    Extension(config): Extension<Config>,
    AuthBearer(token): AuthBearer,
) -> RouteResult<Json<user::Model>, String> {
    // Verify JWT token and extract claims
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

    // Find a user by user ID
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

/// Create new user and link credentials
pub async fn create_new_user(
    Extension(config): Extension<Config>,
    Extension(db): Extension<DatabaseConnection>,
    AuthBearer(token): AuthBearer,
    Json(payload): Json<CreateUserPayload>,
) -> RouteResult<(), String> {
    // Verify token
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

    // Find a user by user ID
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

    // Check whether the user is admin
    if !user.admin {
        return Err((StatusCode::FORBIDDEN, "You are not admin".to_string()));
    }

    // Check whether the username is taken
    match user::Entity::find()
        .filter(user::Column::Nickname.like(&payload.username))
        .one(&db)
        .await
    {
        Ok(res) => {
            if let Some(_) = res {
                return Err((StatusCode::CONFLICT, "Username already taken".to_string()));
            }
        }
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    };

    // Create new user
    let new_user_data = user::ActiveModel {
        nickname: Set(payload.username.clone()),
        admin: Set(false),

        ..Default::default()
    };
    if let Err(err) = user::Entity::insert(new_user_data).exec(&db).await {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()));
    };

    // Fetch new user
    let new_user_res = user::Entity::find()
        .filter(user::Column::Nickname.like(&payload.username.clone()))
        .one(&db)
        .await;

    let new_user = match new_user_res {
        Ok(res) => res.unwrap(),
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    };

    // Create new credential
    let salt = utils::generate_salt();
    let new_cred_data = credential::ActiveModel {
        user_id: Set(new_user.id),
        password: Set(utils::hash_password(payload.password, salt.clone())),
        salt: Set(salt.clone()),
        ..Default::default()
    };
    if let Err(err) = credential::Entity::insert(new_cred_data).exec(&db).await {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()));
    };

    Ok((StatusCode::CREATED, ()))
}
