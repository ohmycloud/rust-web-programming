use crate::todo_items::enums::TaskStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewToDoItem {
    pub title: String,
    pub status: TaskStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::FromRow)]
#[cfg_attr(feature = "sqlx-postgres", derive(sqlx::FromRow))]
pub struct ToDoItem {
    pub id: String,
    pub title: String,
    pub status: String,
}
