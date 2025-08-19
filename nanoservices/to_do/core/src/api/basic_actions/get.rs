use crate::structs::{AllToDoItems, ToDoItem};
use dal::json_file::get_all as get_all_handle;
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};

pub async fn get_all() -> Result<AllToDoItems, NanoServiceError> {
    let hashmap = get_all_handle::<ToDoItem>()?;
    Ok(AllToDoItems::from_hashmap(hashmap))
}

pub async fn get_by_name(name: &str) -> Result<ToDoItem, NanoServiceError> {
    let item = get_all_handle::<ToDoItem>()?
        .remove(name)
        .ok_or(NanoServiceError::new(
            format!("Item with name {} not found", name),
            NanoServiceErrorStatus::NotFound,
        ))?;
    Ok(item)
}
