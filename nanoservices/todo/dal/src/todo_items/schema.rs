use super::enums::TaskStatus;
use glue::errors::NanoServiceError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewToDoItem {
    pub title: String,
    pub status: TaskStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "sqlx-postgres", derive(sqlx::FromRow))]
pub struct ToDoItem {
    pub id: String,
    pub title: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllToDoItems {
    pub pending: Vec<ToDoItem>,
    pub done: Vec<ToDoItem>,
}

impl AllToDoItems {
    pub fn from_vec(all_items: Vec<ToDoItem>) -> Result<AllToDoItems, NanoServiceError> {
        let mut pending = Vec::new();
        let mut done = Vec::new();

        for item in all_items {
            match TaskStatus::from_string(&item.status).unwrap() {
                TaskStatus::PENDING => pending.push(item),
                TaskStatus::DONE => done.push(item),
            }
        }
        Ok(AllToDoItems { pending, done })
    }
}
