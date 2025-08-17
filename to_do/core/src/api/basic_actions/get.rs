use crate::structs::{AllToDoItems, ToDoItem};
use dal::json_file::get_all as get_all_handle;

pub async fn get_all() -> Result<AllToDoItems, String> {
    let hashmap = get_all_handle::<ToDoItem>()?;
    Ok(AllToDoItems::from_hashmap(hashmap))
}
