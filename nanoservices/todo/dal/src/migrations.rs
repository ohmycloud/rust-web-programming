use crate::connections::sqlx_postgres::SQLX_POSTGRES_POOL;

pub async fn run_migrations() -> Result<(), sqlx::Error> {
    println!("Migrating todo databases...");
    let mut migrations = sqlx::migrate!("../../../migrations");
    migrations.set_ignore_missing(true);

    let result = migrations.run(&*SQLX_POSTGRES_POOL).await?;
    println!("todo database migrations completed: {:?}", result);

    Ok(())
}
