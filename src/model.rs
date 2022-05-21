use sqlx::postgres::PgPool;

use crate::users_proto::User as UserMessage;

type FetchResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(sqlx::FromRow)]
pub struct UserModel {
    pub id: i32,
    pub nickname: String,
    pub admin: bool,
}

impl UserModel {
    pub async fn get_all(pool: &PgPool) -> FetchResult<Vec<UserModel>> {
        Ok(sqlx::query_as::<_, UserModel>("SELECT * FROM users")
            .fetch_all(pool)
            .await?)
    }

    pub async fn get_by_id(id: i32, pool: &PgPool) -> FetchResult<Option<UserModel>> {
        Ok(
            sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE id = $1")
                .bind(id)
                .fetch_optional(pool)
                .await?,
        )
    }
}

impl UserModel {
    pub fn into_message(&self) -> UserMessage {
        UserMessage {
            id: self.id,
            nickname: self.nickname.to_owned(),
        }
    }
}
