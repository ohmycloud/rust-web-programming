use crate::enums::TaskStatus;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToDoItem {
    pub title: String,
    pub status: TaskStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllToDoItems {
    pub pending: Vec<ToDoItem>,
    pub done: Vec<ToDoItem>,
}

impl AllToDoItems {
    pub fn from_hashmap(all_items: HashMap<String, ToDoItem>) -> Self {
        let mut pending = Vec::new();
        let mut done = Vec::new();

        for (_, item) in all_items {
            match item.status {
                TaskStatus::PENDING => pending.push(item),
                TaskStatus::DONE => done.push(item),
            }
        }

        Self { pending, done }
    }
}

impl fmt::Display for ToDoItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.status {
            TaskStatus::PENDING => write!(f, "Pending: {}", self.title),
            TaskStatus::DONE => write!(f, "Done: {}", self.title),
        }
    }
}
