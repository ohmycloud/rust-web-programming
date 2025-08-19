use dal::json_file::delete_one;
use glue::errors::NanoServiceError;

use crate::structs::ToDoItem;

pub async fn delete(id: &str) -> Result<(), NanoServiceError> {
    delete_one::<ToDoItem>(id)
}
