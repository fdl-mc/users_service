use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use tonic::{Request, Response, Status};

use crate::models::{credential, user};
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
    pub conn: DatabaseConnection,
}

#[tonic::async_trait]
impl UsersServiceTrait for UsersService {
    async fn get_all_users(
        &self,
        _request: Request<GetAllUsersRequest>,
    ) -> Result<Response<GetAllUsersReply>, Status> {
        // Fetch usera
        let users = match user::Entity::find().all(&self.conn).await {
            Ok(res) => res,
            Err(err) => return Err(Status::internal(err.to_string())),
        };

        // Prepare reply
        let reply = GetAllUsersReply {
            users: users.iter().map(user::Model::into_message).collect(),
        };
        Ok(Response::new(reply))
    }

    async fn get_user_by_id(
        &self,
        request: Request<GetUserByIdRequest>,
    ) -> Result<Response<GetUserByIdReply>, Status> {
        let message = request.get_ref();

        // Fetch user
        let user = match user::Entity::find_by_id(message.id).one(&self.conn).await {
            Ok(res) => match res {
                Some(res) => res,
                None => return Err(Status::not_found("User not found")),
            },
            Err(err) => return Err(Status::internal(err.to_string())),
        };

        // Prepare reply
        let reply = GetUserByIdReply {
            user: Some(user.into_message()),
        };
        Ok(Response::new(reply))
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
        let user = match user::Entity::find_by_id(claims.user_id)
            .one(&self.conn)
            .await
        {
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
        let message = request.get_ref();

        // Extract query type from request
        let query = match &message.query {
            Some(res) => res,
            None => return Err(Status::invalid_argument("Query field should not be empty")),
        };

        // Make database query based on selected query type
        let users_query_filter = match query {
            find_users_request::Query::Nickname(nickname) => {
                user::Column::Nickname.starts_with(&nickname)
            }
        };

        // Find all users matching query
        let users = match user::Entity::find()
            .filter(users_query_filter)
            .all(&self.conn)
            .await
        {
            Ok(res) => res,
            Err(err) => return Err(Status::internal(err.to_string())),
        };

        // Prepare and send reply
        let reply = FindUsersReply {
            users: users.iter().map(user::Model::into_message).collect(),
        };
        Ok(Response::new(reply))
    }

    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginReply>, Status> {
        let message = request.get_ref();

        // Get user
        let user = match user::Entity::find()
            .filter(user::Column::Nickname.eq(message.username.to_owned()))
            .one(&self.conn)
            .await
        {
            Ok(res) => match res {
                Some(res) => res,
                None => return Err(Status::unauthenticated("Wrong username or password")),
            },
            Err(err) => return Err(Status::internal(err.to_string())),
        };

        // Get their credentials
        let credentials = match credential::Entity::find()
            .filter(credential::Column::UserId.eq(user.id))
            .one(&self.conn)
            .await
        {
            Ok(res) => match res {
                Some(res) => res,
                None => return Err(Status::internal("lol what")),
            },
            Err(err) => return Err(Status::internal(err.to_string())),
        };

        // Verify password
        if !credentials.verify_password(message.password.to_owned()) {
            return Err(Status::unauthenticated("Wrong username or password"));
        }

        // If OK, then...

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
        let credential = match credential::Entity::find()
            .filter(credential::Column::UserId.eq(claims.user_id))
            .one(&self.conn)
            .await
        {
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
        let mut credential: credential::ActiveModel = credential.into();
        credential.salt = Set(salt);
        credential.password = Set(password);

        match credential.update(&self.conn).await {
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
        let user = match user::Entity::find_by_id(claims.user_id)
            .one(&self.conn)
            .await
        {
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
        match user::Entity::find()
            .filter(user::Column::Nickname.eq(message.username.clone()))
            .one(&self.conn)
            .await
        {
            Ok(res) => match res {
                Some(_) => return Err(Status::already_exists("Username already taken")),
                None => (),
            },
            Err(err) => return Err(Status::internal(err.to_string())),
        };

        // Create new user
        let user = user::ActiveModel {
            nickname: Set(message.username.clone()),
            ..Default::default()
        };

        let user = match user.insert(&self.conn).await {
            Ok(res) => res,
            Err(err) => return Err(Status::internal(err.to_string())),
        };

        // Create new credentials
        let salt = utils::crypto::generate_salt();
        let password = utils::crypto::hash_password(message.password.clone(), salt.clone());
        let credentials = credential::ActiveModel {
            user_id: Set(user.id),
            password: Set(password),
            salt: Set(salt),
            ..Default::default()
        };

        match credentials.insert(&self.conn).await {
            Ok(_) => (),
            Err(err) => return Err(Status::internal(err.to_string())),
        };

        // Hooray!
        Ok(Response::new(CreateUserReply {}))
    }
}
