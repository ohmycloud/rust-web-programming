#[cfg(feature = "sqlx-postgres")]
use crate::connections::sqlx_postgres::SQLX_POSTGRES_POOL;
#[cfg(feature = "json-file")]
use crate::json_file::{get_all, save_all};
#[cfg(feature = "json-file")]
use crate::todo_items::descriptors::JsonFileDescriptor;
#[cfg(feature = "sqlx-postgres")]
use crate::todo_items::descriptors::SqlxPostGresDescriptor;

use crate::todo_items::schema::ToDoItem;
#[cfg(any(feature = "json-file", feature = "sqlx-postgres"))]
use glue::errors::NanoServiceError;
use glue::errors::NanoServiceErrorStatus;
use std::collections::HashMap;

pub trait UpdateOne {
    fn update_one(
        item: ToDoItem,
    ) -> impl Future<Output = Result<ToDoItem, NanoServiceError>> + Send;
}

#[cfg(feature = "sqlx-postgres")]
impl UpdateOne for SqlxPostGresDescriptor {
    fn update_one(
        item: ToDoItem,
    ) -> impl Future<Output = Result<ToDoItem, NanoServiceError>> + Send {
        sqlx_postgres_update_one(item)
    }
}

#[cfg(feature = "json-file")]
impl UpdateOne for JsonFileDescriptor {
    fn update_one(
        item: ToDoItem,
    ) -> impl Future<Output = Result<ToDoItem, NanoServiceError>> + Send {
        json_file_update_one(item)
    }
}

#[cfg(feature = "sqlx-postgres")]
async fn sqlx_postgres_update_one(item: ToDoItem) -> Result<ToDoItem, NanoServiceError> {
    let item = sqlx::query_as::<_, ToDoItem>(
        "UPDATE todo_items set title = $1, status = $2 WHERE id = $3 RETURNING *",
    )
    .bind(item.title)
    .bind(item.status)
    .bind(item.id)
    .fetch_one(&*SQLX_POSTGRES_POOL)
    .await
    .map_err(|err| NanoServiceError::new(err.to_string(), NanoServiceErrorStatus::Unknown))?;

    Ok(item)
}

#[cfg(feature = "json-file")]
async fn json_file_update_one(item: ToDoItem) -> Result<ToDoItem, NanoServiceError> {
    let mut tasks = get_all::<ToDoItem>().unwrap_or_else(|_| HashMap::new());
    if !tasks.contains_key(&item.title) {
        return Err(NanoServiceError::new(
            format!("Item with name {} not found", item.title),
            NanoServiceErrorStatus::NotFound,
        ));
    }

    tasks.insert(item.title.clone(), item.clone());
    let _ = save_all(&tasks)?;

    Ok(item)
}
