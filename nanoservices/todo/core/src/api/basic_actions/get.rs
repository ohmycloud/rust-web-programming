use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use todo_dal::todo_items::schema::{AllToDoItems, ToDoItem};
use todo_dal::{json_file::get_all as get_all_handle, todo_items::transactions::get::GetAll};

pub async fn get_all<T: GetAll>() -> Result<AllToDoItems, NanoServiceError> {
    let all_items = T::get_all().await?;
    AllToDoItems::from_vec(all_items)
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
