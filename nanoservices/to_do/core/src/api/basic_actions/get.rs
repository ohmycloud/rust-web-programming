use crate::structs::{AllToDoItems, ToDoItem};
use dal::json_file::get_all as get_all_handle;
use glue::errors::NanoServiceError;

pub async fn get_all() -> Result<AllToDoItems, NanoServiceError> {
    let hashmap = get_all_handle::<ToDoItem>()?;
    Ok(AllToDoItems::from_hashmap(hashmap))
}
