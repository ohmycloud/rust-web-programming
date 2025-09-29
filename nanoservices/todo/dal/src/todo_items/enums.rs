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

impl TaskStatus {
    pub fn from_string(status: &String) -> Result<TaskStatus, String> {
        match status.to_uppercase().as_str() {
            "DONE" => Ok(TaskStatus::DONE),
            "PENDING" => Ok(TaskStatus::PENDING),
            _ => Err(format!("Invalid status: {}", status)),
        }
    }
}
