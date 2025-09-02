use glue::errors::NanoServiceError;
use todo_dal::json_file::delete_one;

use crate::structs::ToDoItem;

pub async fn delete(id: &str) -> Result<(), NanoServiceError> {
    delete_one::<ToDoItem>(id)
}
