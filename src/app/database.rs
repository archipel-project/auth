pub type DbPool = sqlx::postgres::PgPool;

pub async fn builder() -> Result<DbPool, sqlx::Error> {
    let connectspec = dotenv::var("DATABASE_URL").expect("DATABASE_URL is not set");
    DbPool::connect(&connectspec).await
}

pub async fn migrate(pool: &DbPool) -> Result<(), sqlx::Error> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}