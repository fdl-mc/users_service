use crate::models::{credential, user};
use sea_orm::{prelude::*, ConnectionTrait, Schema};

async fn migrate<E>(db: DatabaseConnection, entity: E)
where
    E: EntityTrait,
{
    let backend = db.get_database_backend();
    let schema = Schema::new(backend);
    let table = schema.create_table_from_entity(entity);
    let statement = backend.build(&table);
    let result = db.execute(statement).await;

    match result {
        Ok(_) => {
            tracing::info!(target: "Migration", "Successfuly migrated {}", entity.module_name());
        }
        Err(err) => {
            tracing::error!(target: "Migration", "An error occured while migrating {}: {}", entity.module_name(), err);
        }
    };
}

pub async fn migrate_all(db: DatabaseConnection) {
    migrate(db.clone(), user::Entity).await;
    migrate(db.clone(), credential::Entity).await;
}
