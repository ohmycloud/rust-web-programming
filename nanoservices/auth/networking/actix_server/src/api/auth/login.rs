use crate::extract_auth::extract_credientials;
use actix_web::HttpResponse;
use auth_core::api::auth::login as core_login;
use auth_dal::users::GetByEmail;
use glue::errors::NanoServiceError;

pub async fn login<T: GetByEmail>(
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, NanoServiceError> {
    let credentials = extract_credientials(req).await?;
    let token = core_login::<T>(credentials.email, credentials.password).await?;

    Ok(HttpResponse::Ok().json(token))
}
