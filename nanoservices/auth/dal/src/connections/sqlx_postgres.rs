use std::{env, sync::LazyLock};

use sqlx::{PgPool, postgres::PgPoolOptions};

pub static SQLX_POSTGRES_POOL: LazyLock<PgPool> = LazyLock::new(|| {
    let connection_string = env::var("AUTH_DB_URL").unwrap();
    let max_connections = match std::env::var("AUTH_MAX_CONNECTIONS") {
        Ok(val) => val,
        Err(_) => "5".to_string(),
    }
    .trim()
    .parse::<u32>()
    .map_err(|_| "Could not parse max connections".to_string())
    .unwrap();

    let pool = PgPoolOptions::new().max_connections(max_connections);

    pool.connect_lazy(&connection_string)
        .expect("Failed to create pool")
});
