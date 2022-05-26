use tonic::{Request, Response, Status};

use crate::models::{CredentialModel, UserModel};
use crate::users_proto::users_server::Users as UsersServiceTrait;
use crate::users_proto::{
    find_users_request, ChangePasswordReply, ChangePasswordRequest, FindUsersReply,
    FindUsersRequest, GetAllUsersReply, GetAllUsersRequest, GetSelfUserReply, GetSelfUserRequest,
    GetUserByIdReply, GetUserByIdRequest, LoginReply, LoginRequest,
};
use crate::{Claims, Config};

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
        unimplemented!()
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
        unimplemented!()
    }
}
