use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "credentials")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub password: String,
    pub salt: String,
}

impl Model {
    pub fn verify_password(&self, password_to_verify: String) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(&password_to_verify);
        hasher.update(&self.salt);
        let password_to_verify = format!("{:x}", hasher.finalize());

        self.password == password_to_verify
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
