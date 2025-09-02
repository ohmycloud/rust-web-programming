use std::{env, sync::LazyLock};

use sqlx::{PgPool, postgres::PgPoolOptions};

pub static SQLX_POSTGRES_POOL: LazyLock<PgPool> = LazyLock::new(|| {
    let connection_string = env::var("DATABASE_URL").expect("expect database url.");
    let max_connections = match env::var("MAX_CONNECTIONS") {
        Ok(value) => value,
        Err(_) => "5".to_string(),
    }
    .trim()
    .parse::<u32>()
    .map_err(|_| "Couldn't parse max connections".to_string())
    .unwrap();

    let pool = PgPoolOptions::new().max_connections(max_connections);
    pool.connect_lazy(&connection_string)
        .expect("Failed to connect to PostgreSQL")
});
