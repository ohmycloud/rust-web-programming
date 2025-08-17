use std::fmt;

pub enum TaskStatus {
    DONE,
    PENDING,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskStatus::DONE => write!(f, "Done"),
            TaskStatus::PENDING => write!(f, "Pending"),
        }
    }
}
