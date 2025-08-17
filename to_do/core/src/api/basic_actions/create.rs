use crate::enums::TaskStatus;
use crate::structs::{done::Done, pending::Pending};
use std::fmt;

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

pub fn create(title: &str, status: TaskStatus) -> ItemTypes {
    match status {
        TaskStatus::DONE => ItemTypes::Done(Done::new(title)),
        TaskStatus::PENDING => ItemTypes::Pending(Pending::new(title)),
    }
}
