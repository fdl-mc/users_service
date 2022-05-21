use tonic::{Request, Response, Status};

use crate::model::UserModel;
use crate::users_proto::users_server::Users as UsersServiceTrait;
use crate::users_proto::{
    ChangePasswordRequest, ChangePasswordResponse, FindUsersRequest, FindUsersResponse,
    GetAllUsersRequest, GetAllUsersResponse, GetSelfUserRequest, GetSelfUserResponse,
    GetUserByIdRequest, GetUserByIdResponse, LoginRequest, LoginResponse,
};

#[derive(Debug)]
pub struct UsersService {
    pub pool: sqlx::PgPool,
}

#[tonic::async_trait]
impl UsersServiceTrait for UsersService {
    async fn get_all_users(
        &self,
        _request: Request<GetAllUsersRequest>,
    ) -> Result<Response<GetAllUsersResponse>, Status> {
        match UserModel::get_all(&self.pool.clone()).await {
            Ok(res) => Ok(Response::new(GetAllUsersResponse {
                users: res.iter().map(|x| x.into_message()).collect(),
            })),
            Err(err) => Err(Status::internal(err.to_string())),
        }
    }

    async fn get_user_by_id(
        &self,
        request: Request<GetUserByIdRequest>,
    ) -> Result<Response<GetUserByIdResponse>, Status> {
        unimplemented!()
    }

    async fn get_self_user(
        &self,
        request: Request<GetSelfUserRequest>,
    ) -> Result<Response<GetSelfUserResponse>, Status> {
        unimplemented!()
    }

    async fn find_users(
        &self,
        request: Request<FindUsersRequest>,
    ) -> Result<Response<FindUsersResponse>, Status> {
        unimplemented!()
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        unimplemented!()
    }

    async fn change_password(
        &self,
        request: Request<ChangePasswordRequest>,
    ) -> Result<Response<ChangePasswordResponse>, Status> {
        unimplemented!()
    }
}
