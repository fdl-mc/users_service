use tonic::{Request, Response, Status};

use crate::models::{CredentialModel, UserModel};
use crate::proto::users::users_server::Users as UsersServiceTrait;
use crate::proto::users::{
    find_users_request, ChangePasswordReply, ChangePasswordRequest, CreateUserReply,
    CreateUserRequest, FindUsersReply, FindUsersRequest, GetAllUsersReply, GetAllUsersRequest,
    GetSelfUserReply, GetSelfUserRequest, GetUserByIdReply, GetUserByIdRequest, LoginReply,
    LoginRequest,
};
use crate::{utils, Claims, Config};

#[derive(Debug)]
pub struct UsersService {
    pub config: Config,
    pub pool: sqlx::PgPool,
}

#[tonic::async_trait]
impl UsersServiceTrait for UsersService {
    async fn get_all_users(
        &self,
        _request: Request<GetAllUsersRequest>,
    ) -> Result<Response<GetAllUsersReply>, Status> {
        let res = UserModel::get_all(&self.pool.clone()).await;

        match res {
            Ok(res) => {
                let users = res.iter().map(UserModel::into_message).collect();
                let reply = GetAllUsersReply { users };
                Ok(Response::new(reply))
            }
            Err(err) => Err(Status::internal(err.to_string())),
        }
    }

    async fn get_user_by_id(
        &self,
        request: Request<GetUserByIdRequest>,
    ) -> Result<Response<GetUserByIdReply>, Status> {
        let id = request.into_inner().id;
        let res = UserModel::get_by_id(id, &self.pool.clone()).await;

        match res {
            Ok(res) => match res {
                Some(res) => {
                    let user = res.into_message();
                    let reply = GetUserByIdReply { user: Some(user) };
                    Ok(Response::new(reply))
                }
                None => Err(Status::not_found("User not found")),
            },
            Err(err) => Err(Status::internal(err.to_string())),
        }
    }

    async fn get_self_user(
        &self,
        request: Request<GetSelfUserRequest>,
    ) -> Result<Response<GetSelfUserReply>, Status> {
        // Extract token from metadata
        let token = match request.metadata().get("x-token") {
            Some(res) => res.to_str().unwrap().to_string(),
            None => return Err(Status::unauthenticated("No token provided")),
        };

        // Verify token and extract claims
        let claims = match Claims::from_jwt(token, self.config.jwt_secret.to_owned()) {
            Ok(res) => res,
            Err(_) => return Err(Status::unauthenticated("Token verification failed")),
        };

        // Fetch user from claims' user_id
        let user = match UserModel::get_by_id(claims.user_id, &self.pool.clone()).await {
            Ok(res) => match res {
                Some(res) => res,
                None => return Err(Status::not_found("User not found")),
            },
            Err(err) => return Err(Status::internal(err.to_string())),
        };

        // Make reply and send response
        let reply = GetSelfUserReply {
            user: Some(user.into_message()),
        };
        Ok(Response::new(reply))
    }

    async fn find_users(
        &self,
        request: Request<FindUsersRequest>,
    ) -> Result<Response<FindUsersReply>, Status> {
        match request.into_inner().query {
            Some(res) => match res {
                find_users_request::Query::Nickname(nickname) => {
                    let res = UserModel::search_by_nickname(nickname, &self.pool.clone()).await;
                    match res {
                        Ok(res) => {
                            let users = res.iter().map(UserModel::into_message).collect();
                            let reply = FindUsersReply { users };
                            Ok(Response::new(reply))
                        }
                        Err(err) => Err(Status::internal(err.to_string())),
                    }
                }
            },
            None => Err(Status::invalid_argument("Query field should not be empty")),
        }
    }

    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginReply>, Status> {
        let inner = request.into_inner();

        // Get user
        let user = match UserModel::get_by_nickname(inner.username, &self.pool.clone()).await {
            Ok(res) => match res {
                Some(res) => res,
                None => return Err(Status::unauthenticated("Wrong username or password")),
            },
            Err(err) => return Err(Status::internal(err.to_string())),
        };

        // Get their credentials
        let credentials = match CredentialModel::get_by_user_id(user.id, &self.pool.clone()).await {
            Ok(res) => match res {
                Some(res) => res,
                None => return Err(Status::internal("lol what")),
            },
            Err(err) => return Err(Status::internal(err.to_string())),
        };

        if !credentials.verify_password(inner.password) {
            return Err(Status::unauthenticated("Wrong username or password"));
        }

        // Make JWT claims
        let claims = Claims {
            user_id: user.id,
            exp: 2147483647,
        };

        // Encode to string token
        let token = match claims.to_jwt(self.config.jwt_secret.to_owned()) {
            Ok(res) => res,
            Err(err) => return Err(Status::internal(err.to_string())),
        };

        // Make and send a reply
        let reply = LoginReply { token };
        Ok(Response::new(reply))
    }

    async fn change_password(
        &self,
        request: Request<ChangePasswordRequest>,
    ) -> Result<Response<ChangePasswordReply>, Status> {
        // Extract token from metadata
        let token = match request.metadata().get("x-token") {
            Some(res) => res.to_str().unwrap().to_string(),
            None => return Err(Status::unauthenticated("No token provided")),
        };

        // Verify token and extract claims
        let claims = match Claims::from_jwt(token, self.config.jwt_secret.to_owned()) {
            Ok(res) => res,
            Err(_) => return Err(Status::unauthenticated("Token verification failed")),
        };

        // Fetch credentials from claims' user_id
        let mut credential =
            match CredentialModel::get_by_user_id(claims.user_id, &self.pool.clone()).await {
                Ok(res) => match res {
                    Some(res) => res,
                    None => return Err(Status::not_found("Credentials not found")),
                },
                Err(err) => return Err(Status::internal(err.to_string())),
            };

        // Generate new salt and hash the password
        let salt = utils::crypto::generate_salt();
        let password =
            utils::crypto::hash_password(request.into_inner().new_password, salt.clone());

        // Update fileds
        credential.salt = salt;
        credential.password = password;

        match credential.update_all(&self.pool.clone()).await {
            Ok(_) => (),
            Err(err) => return Err(Status::internal(err.to_string())),
        };

        Ok(Response::new(ChangePasswordReply {}))
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserReply>, Status> {
        let message = request.get_ref();
        let metadata = request.metadata();

        // Extract token from metadata
        let token = match metadata.get("x-token") {
            Some(res) => res.to_str().unwrap().to_string(),
            None => return Err(Status::unauthenticated("No token provided")),
        };

        // Verify token and extract claims
        let claims = match Claims::from_jwt(token, self.config.jwt_secret.to_owned()) {
            Ok(res) => res,
            Err(_) => return Err(Status::unauthenticated("Token verification failed")),
        };

        // Fetch user from credentials
        let user = match UserModel::get_by_id(claims.user_id, &self.pool.clone()).await {
            Ok(res) => match res {
                Some(res) => res,
                None => return Err(Status::not_found("User not found")),
            },
            Err(err) => return Err(Status::internal(err.to_string())),
        };

        // Check is admin
        if !user.admin {
            return Err(Status::permission_denied("cope harder"));
        }

        // Validate input data
        if message.username.is_empty() || message.password.is_empty() {
            return Err(Status::invalid_argument("Invalid data"));
        }

        // Check whether the username is already taken
        match UserModel::get_by_nickname(message.username.clone(), &self.pool.clone()).await {
            Ok(res) => match res {
                Some(_) => return Err(Status::already_exists("Username already taken")),
                None => (),
            },
            Err(err) => return Err(Status::internal(err.to_string())),
        };

        // Create new user
        let mut user = UserModel {
            nickname: message.username.clone(),
            admin: false,
            ..Default::default()
        };

        match user.insert(&self.pool.clone()).await {
            Ok(_) => (),
            Err(err) => return Err(Status::internal(err.to_string())),
        };

        // Create new credentials
        let salt = utils::crypto::generate_salt();
        let password = utils::crypto::hash_password(message.password.clone(), salt.clone());
        let mut credentials = CredentialModel {
            user_id: user.id,
            password,
            salt,
            ..Default::default()
        };

        match credentials.insert(&self.pool.clone()).await {
            Ok(_) => (),
            Err(err) => return Err(Status::internal(err.to_string())),
        };

        // Hooray!
        Ok(Response::new(CreateUserReply {}))
    }
}
