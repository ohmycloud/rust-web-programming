use std::collections::HashMap;

#[cfg(feature = "sqlx-postgres")]
use crate::connections::sqlx_postgres::SQLX_POSTGRES_POOL;
#[cfg(feature = "json-file")]
use crate::json_file::{get_all, save_all};

#[cfg(feature = "sqlx-postgres")]
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};

#[cfg(feature = "json-file")]
use crate::todo_items::descriptors::JsonFileDescriptor;
#[cfg(feature = "sqlx-postgres")]
use crate::todo_items::descriptors::SqlxPostGresDescriptor;
use crate::todo_items::schema::{NewToDoItem, ToDoItem};

pub trait SaveOne {
    fn save_one(
        item: NewToDoItem,
    ) -> impl Future<Output = Result<ToDoItem, NanoServiceError>> + Send;
}

#[cfg(feature = "sqlx-postgres")]
impl SaveOne for SqlxPostGresDescriptor {
    fn save_one(
        item: NewToDoItem,
    ) -> impl Future<Output = Result<ToDoItem, NanoServiceError>> + Send {
        sqlx_postgres_save_one(item)
    }
}

#[cfg(feature = "json-file")]
impl SaveOne for JsonFileDescriptor {
    fn save_one(
        item: NewToDoItem,
    ) -> impl Future<Output = Result<ToDoItem, NanoServiceError>> + Send {
        json_file_save_one(item)
    }
}

#[cfg(feature = "sqlx-postgres")]
async fn sqlx_postgres_save_one(item: NewToDoItem) -> Result<ToDoItem, NanoServiceError> {
    let item = sqlx::query_as::<_, ToDoItem>(
        r#"
        INSERT INTO todo_items (title, status)
        VALUES ($1, $2)
        RETURNING *
        "#,
    )
    .bind(item.title)
    .bind(item.status)
    .fetch_one(&*SQLX_POSTGRES_POOL)
    .await
    .map_err(|e| NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::Unknown))?;

    Ok(item)
}

#[cfg(feature = "json-file")]
async fn json_file_save_one(item: NewToDoItem) -> Result<ToDoItem, NanoServiceError> {
    let mut tasks = get_all::<ToDoItem>().unwrap_or_else(|_| HashMap::new());
    let todo_item = ToDoItem {
        id: 1.to_string(),
        title: item.title,
        status: item.status.into(),
    };
    tasks.insert(todo_item.title.clone(), todo_item.clone());
    let _ = save_all(&tasks)?;

    Ok(todo_item)
}
