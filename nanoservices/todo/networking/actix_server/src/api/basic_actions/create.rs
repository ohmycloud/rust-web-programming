use actix_web::{HttpResponse, web::Json};
use glue::{errors::NanoServiceError, token::HeaderToken};
use todo_core::api::basic_actions::{create::create as create_core, get::get_all as get_all_core};
use todo_dal::todo_items::{
    schema::NewToDoItem,
    transactions::{create::SaveOne, get::GetAll},
};

pub async fn create<T: SaveOne + GetAll>(
    token: HeaderToken,
    body: Json<NewToDoItem>,
) -> Result<HttpResponse, NanoServiceError> {
    println!("Token: {}", token.message);
    let _ = create_core::<T>(body.into_inner()).await?;
    Ok(HttpResponse::Created().json(get_all_core::<T>().await?))
}
