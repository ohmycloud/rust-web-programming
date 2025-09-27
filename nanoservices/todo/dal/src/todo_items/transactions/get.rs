use std::collections::HashMap;

use glue::errors::{NanoServiceError, NanoServiceErrorStatus};

use crate::{
    connections::sqlx_postgres::SQLX_POSTGRES_POOL,
    json_file::get_all,
    todo_items::{
        descriptors::{JsonFileDescriptor, SqlxPostGresDescriptor},
        schema::ToDoItem,
    },
};

pub trait GetAll {
    fn get_all() -> impl Future<Output = Result<Vec<ToDoItem>, NanoServiceError>> + Send;
}

impl GetAll for SqlxPostGresDescriptor {
    fn get_all() -> impl Future<Output = Result<Vec<ToDoItem>, NanoServiceError>> + Send {
        sqlx_postgres_get_all()
    }
}

// #[cfg(feature = "sqlx-postgres")]
async fn sqlx_postgres_get_all() -> Result<Vec<ToDoItem>, NanoServiceError> {
    let items = sqlx::query_as("SELECT * from todo_items")
        .fetch_all(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|err| NanoServiceError::new(err.to_string(), NanoServiceErrorStatus::Unknown))?;
    Ok(items)
}

impl GetAll for JsonFileDescriptor {
    fn get_all() -> impl Future<Output = Result<Vec<ToDoItem>, NanoServiceError>> + Send {
        json_file_get_all()
    }
}

#[cfg(feature = "json-file")]
async fn json_file_get_all() -> Result<Vec<ToDoItem>, NanoServiceError> {
    let items = get_all::<ToDoItem>().unwrap_or_else(|_| HashMap::new());
    let items = items.into_values().collect();
    Ok(items)
}
