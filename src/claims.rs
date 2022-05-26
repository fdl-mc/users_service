use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i32,
    pub exp: i32,
}

impl Claims {
    pub fn from_jwt(token: String, secret: String) -> Result<Claims, Box<dyn std::error::Error>> {
        let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
        validation.validate_exp = false;
        Ok(jsonwebtoken::decode::<Claims>(
            &token,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
            &validation,
        )?
        .claims)
    }

    pub fn to_jwt(&self, secret: String) -> Result<String, Box<dyn std::error::Error>> {
        Ok(jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            self,
            &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
        )?)
    }
}
