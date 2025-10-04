use actix_web::{HttpResponse, web::Json};
use auth_core::api::users::create::{CreateUser, create as create_core};
use auth_dal::SaveOne;
use glue::errors::NanoServiceError;

pub async fn create<T: SaveOne>(user: Json<CreateUser>) -> Result<HttpResponse, NanoServiceError> {
    let _ = create_core::<T>(user.into_inner()).await?;
    Ok(HttpResponse::Created().finish())
}
