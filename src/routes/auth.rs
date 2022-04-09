use crate::{
    models::{
        config::Config,
        credential,
        jwt_claims::Claims,
        payloads::{ChangePasswordPayload, LoginPayload},
        responses::LoginResponse,
        user,
    },
    utils::{generate_salt, hash_password},
};
use axum::{extract::Extension, http::StatusCode, Json};
use axum_auth::AuthBearer;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

type RouteResult<T, E> = Result<(StatusCode, T), (StatusCode, E)>;

/// Verify credentials and return an access token
pub async fn login(
    Extension(db): Extension<DatabaseConnection>,
    Extension(config): Extension<Config>,
    Json(payload): Json<LoginPayload>,
) -> RouteResult<Json<LoginResponse>, String> {
    // Find a user by username
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

    // Find a credential by user ID
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

    // Verify password
    if hash_password(payload.password, credential.salt.clone()) != credential.password {
        return Err((StatusCode::FORBIDDEN, "Incorrect password".to_string()));
    }

    // Make token
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

/// Change account password
pub async fn change_password(
    Extension(config): Extension<Config>,
    Extension(db): Extension<DatabaseConnection>,
    AuthBearer(token): AuthBearer,
    Json(payload): Json<ChangePasswordPayload>,
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
