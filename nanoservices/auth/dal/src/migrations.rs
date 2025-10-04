use crate::connections::SQLX_POSTGRES_POOL;

pub async fn run_migrations() {
    println!("Migrating todo database...");
    let mut migrations = sqlx::migrate!("./migrations");
    migrations.set_ignore_missing(true);
    let result = migrations.run(&*SQLX_POSTGRES_POOL).await.unwrap();
    println!("todo database migrations completed: {:?}", result);
}
