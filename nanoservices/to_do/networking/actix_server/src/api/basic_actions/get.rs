use actix_web::HttpResponse;
use core::api::basic_actions::get::get_all as get_all_core;
use glue::errors::NanoServiceError;

pub async fn get_all() -> Result<HttpResponse, NanoServiceError> {
    Ok(HttpResponse::Ok().json(get_all_core().await?))
}
