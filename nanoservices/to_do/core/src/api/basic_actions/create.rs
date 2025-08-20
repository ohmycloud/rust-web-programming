use crate::structs::ToDoItem;
use glue::errors::NanoServiceError;
use todo_dal::json_file::save_one;

pub async fn create(item: ToDoItem) -> Result<ToDoItem, NanoServiceError> {
    let _ = save_one(&item.title.to_string(), &item)?;

    Ok(item)
}
