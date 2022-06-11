use sqlx::postgres::PgPool;

use crate::proto::users::User as UserMessage;

type FetchResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(sqlx::FromRow, Default)]
pub struct UserModel {
    pub id: i32,
    pub nickname: String,
    pub admin: bool,
}

impl UserModel {
    pub async fn insert(&mut self, pool: &PgPool) -> FetchResult<()> {
        let res = sqlx::query_as::<_, UserModel>(
            "INSERT INTO users (nickname, admin) VALUES ($1, $2) RETURNING id, nickname, admin",
        )
        .bind(self.nickname.clone())
        .bind(self.admin)
        .fetch_one(pool)
        .await?;

        self.id = res.id;
        self.nickname = res.nickname;
        self.admin = res.admin;

        Ok(())
    }

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

    pub async fn get_by_nickname(
        nickname: String,
        pool: &PgPool,
    ) -> FetchResult<Option<UserModel>> {
        Ok(
            sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE nickname = $1")
                .bind(nickname)
                .fetch_optional(pool)
                .await?,
        )
    }

    pub async fn search_by_nickname(
        nickname: String,
        pool: &PgPool,
    ) -> FetchResult<Vec<UserModel>> {
        let nickname_wildcard = nickname + "%";
        Ok(
            sqlx::query_as::<_, UserModel>("SELECT * FROM users WHERE nickname LIKE $1")
                .bind(nickname_wildcard)
                .fetch_all(pool)
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
