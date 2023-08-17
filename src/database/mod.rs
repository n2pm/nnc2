use sqlx::SqlitePool;

pub type Db = SqlitePool;

pub async fn get_db_client() -> sqlx::Result<Db> {
    let db = SqlitePool::connect("sqlite::memory:").await?;
    Ok(db)
}
