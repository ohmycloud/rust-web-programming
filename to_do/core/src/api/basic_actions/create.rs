use crate::enums::TaskStatus;
use crate::structs::{done::Done, pending::Pending};
use dal::json_file::save_one;
use std::fmt;

#[derive(Debug)]
pub enum ItemTypes {
    Done(Done),
    Pending(Pending),
}

impl fmt::Display for ItemTypes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ItemTypes::Done(done) => write!(f, "Done: {}", done.super_struct.title),
            ItemTypes::Pending(pending) => write!(f, "Pending: {}", pending.super_struct.title),
        }
    }
}

pub fn create(title: &str, status: TaskStatus) -> Result<ItemTypes, String> {
    let _ = save_one(&title.to_string(), &status)?;

    match status {
        TaskStatus::DONE => Ok(ItemTypes::Done(Done::new(title))),
        TaskStatus::PENDING => Ok(ItemTypes::Pending(Pending::new(title))),
    }
}
