use glue::errors::NanoServiceError;
use todo_dal::todo_items::{schema::ToDoItem, transactions::update::UpdateOne};

pub async fn update<T: UpdateOne>(item: ToDoItem) -> Result<(), NanoServiceError> {
    let _ = T::update_one(item).await?;

    Ok(())
}
