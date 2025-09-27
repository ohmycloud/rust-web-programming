use crate::{
    connections::sqlx_postgres::SQLX_POSTGRES_POOL,
    json_file::{get_all, save_all},
    todo_items::{
        descriptors::{JsonFileDescriptor, SqlxPostGresDescriptor},
        schema::ToDoItem,
    },
};
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use std::collections::HashMap;

pub trait DeleteOne {
    async fn delete_one(
        title: String,
    ) -> impl Future<Output = Result<ToDoItem, NanoServiceError>> + Send;
}

impl DeleteOne for SqlxPostGresDescriptor {
    async fn delete_one(
        title: String,
    ) -> impl Future<Output = Result<ToDoItem, NanoServiceError>> + Send {
        sqlx_postgres_delete_one(title)
    }
}

impl DeleteOne for JsonFileDescriptor {
    async fn delete_one(
        title: String,
    ) -> impl Future<Output = Result<ToDoItem, NanoServiceError>> + Send {
        json_file_delete_one(title)
    }
}

// #[cfg(feature = "sqlx-postgres")]
async fn sqlx_postgres_delete_one(title: String) -> Result<ToDoItem, NanoServiceError> {
    let item = sqlx::query_as::<_, ToDoItem>("DELETE FROM todo_items WHERE title = $1 RETURNING *")
        .bind(title)
        .fetch_one(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|err| NanoServiceError::new(err.to_string(), NanoServiceErrorStatus::Unknown))?;

    Ok(item)
}

#[cfg(feature = "json-file")]
async fn json_file_delete_one(title: String) -> Result<ToDoItem, NanoServiceError> {
    let mut tasks = get_all::<ToDoItem>().unwrap_or_else(|_| HashMap::new());
    let todo_item = tasks.remove(&title).ok_or_else(|| {
        NanoServiceError::new(
            "Item not found".to_string(),
            NanoServiceErrorStatus::NotFound,
        )
    })?;

    let _ = save_all(&tasks)?;
    Ok(todo_item)
}
