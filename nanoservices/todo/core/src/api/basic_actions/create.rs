use glue::errors::NanoServiceError;
use todo_dal::todo_items::{
    schema::{NewToDoItem, ToDoItem},
    transactions::create::SaveOne,
};

pub async fn create<T: SaveOne>(item: NewToDoItem) -> Result<ToDoItem, NanoServiceError> {
    let created_item = T::save_one(item).await?;

    Ok(created_item)
}
