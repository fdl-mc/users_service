use sqlx::postgres::PgPool;

type FetchResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(sqlx::FromRow)]
pub struct CredentialModel {
    pub id: i32,
    pub user_id: i32,
    pub password: String,
    pub salt: String,
}

impl CredentialModel {
    pub async fn get_by_id(id: i32, pool: &PgPool) -> FetchResult<Option<CredentialModel>> {
        Ok(
            sqlx::query_as::<_, Self>("SELECT * FROM credentials WHERE id = $1")
                .bind(id)
                .fetch_optional(pool)
                .await?,
        )
    }
}
