use glue::errors::NanoServiceError;
use todo_dal::todo_items::transactions::delete::DeleteOne;

pub async fn delete<T: DeleteOne>(id: &str) -> Result<(), NanoServiceError> {
    let _ = T::delete_one(id.to_string()).await?;
    Ok(())
}
