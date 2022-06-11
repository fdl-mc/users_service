use sha2::{Digest, Sha256};
use sqlx::postgres::PgPool;

type FetchResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(sqlx::FromRow, Default)]
pub struct CredentialModel {
    pub id: i32,
    pub user_id: i32,
    pub password: String,
    pub salt: String,
}

impl CredentialModel {
    pub async fn insert(&mut self, pool: &PgPool) -> FetchResult<()> {
        let res = sqlx::query_as::<_, CredentialModel>(
            "INSERT INTO credentials (user_id, password, salt) VALUES ($1, $2, $3) RETURNING id, user_id, password, salt"
        )
        .bind(self.user_id)
        .bind(self.password.clone())
        .bind(self.salt.clone())
        .fetch_one(pool)
        .await?;

        self.id = res.id;
        self.user_id = res.user_id;
        self.password = res.password;
        self.salt = res.salt;

        Ok(())
    }

    pub async fn get_by_user_id(id: i32, pool: &PgPool) -> FetchResult<Option<CredentialModel>> {
        Ok(
            sqlx::query_as::<_, Self>("SELECT * FROM credentials WHERE user_id = $1")
                .bind(id)
                .fetch_optional(pool)
                .await?,
        )
    }

    pub async fn update_all(&self, pool: &PgPool) -> FetchResult<()> {
        sqlx::query("UPDATE credentials SET password = $1, salt = $2 WHERE id = $3")
            .bind(self.password.clone())
            .bind(self.salt.clone())
            .bind(self.id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub fn verify_password(&self, password_to_verify: String) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(&password_to_verify);
        hasher.update(&self.salt);
        let password_to_verify = format!("{:x}", hasher.finalize());

        self.password == password_to_verify
    }
}
