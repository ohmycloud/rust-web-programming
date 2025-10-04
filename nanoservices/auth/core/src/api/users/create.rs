use auth_dal::{NewUser, SaveOne, User};
use glue::errors::NanoServiceError;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub password: String,
}

pub async fn create<T: SaveOne>(user: CreateUser) -> Result<User, NanoServiceError> {
    let user = NewUser::new(user.email, user.password)?;
    let created_item = T::save_one(user).await?;
    Ok(created_item)
}
