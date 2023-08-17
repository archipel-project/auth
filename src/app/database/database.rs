use sea_orm::{Database, DbErr, DatabaseConnection};

pub async fn connect() -> Result<DatabaseConnection, DbErr> {
    let url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL is not set in rnv variables.");

    log::info!("Etablishing connection with database...");
    Database::connect(url).await
}