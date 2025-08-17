use super::super::enums::TaskStatus;

#[derive(Debug)]
pub struct Base {
    pub title: String,
    pub status: TaskStatus,
}
