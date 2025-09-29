use todo_core::api::basic_actions::{get::get_all as get_all_core, update::update as update_core};

use actix_web::{HttpResponse, web::Json};
use glue::errors::NanoServiceError;
use todo_dal::todo_items::{
    schema::ToDoItem,
    transactions::{get::GetAll, update::UpdateOne},
};

pub async fn update<T: UpdateOne + GetAll>(
    body: Json<ToDoItem>,
) -> Result<HttpResponse, NanoServiceError> {
    let _ = update_core::<T>(body.into_inner()).await?;
    Ok(HttpResponse::Ok().json(get_all_core::<T>().await?))
}
