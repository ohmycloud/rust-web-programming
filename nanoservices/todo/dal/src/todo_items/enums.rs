use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type)]
pub enum TaskStatus {
    DONE,
    PENDING,
}

impl From<&str> for TaskStatus {
    fn from(status: &str) -> Self {
        match status {
            "DONE" => TaskStatus::DONE,
            "PENDING" => TaskStatus::PENDING,
            _ => TaskStatus::PENDING,
        }
    }
}

impl From<TaskStatus> for String {
    fn from(status: TaskStatus) -> Self {
        match status {
            TaskStatus::DONE => "DONE".to_string(),
            TaskStatus::PENDING => "PENDING".to_string(),
        }
    }
}
