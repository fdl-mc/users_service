use tonic::{Request, Response, Status};

use crate::model::UserModel;
use crate::users_proto::users_server::Users as UsersServiceTrait;
use crate::users_proto::{
    ChangePasswordReply, ChangePasswordRequest, FindUsersReply, FindUsersRequest, GetAllUsersReply,
    GetAllUsersRequest, GetSelfUserReply, GetSelfUserRequest, GetUserByIdReply, GetUserByIdRequest,
    LoginReply, LoginRequest,
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
    ) -> Result<Response<GetAllUsersReply>, Status> {
        let res = UserModel::get_all(&self.pool.clone()).await;

        match res {
            Ok(res) => {
                let users = res.iter().map(|user| user.into_message()).collect();
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
        unimplemented!()
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
        unimplemented!()
    }

    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginReply>, Status> {
        unimplemented!()
    }

    async fn change_password(
        &self,
        request: Request<ChangePasswordRequest>,
    ) -> Result<Response<ChangePasswordReply>, Status> {
        unimplemented!()
    }
}
