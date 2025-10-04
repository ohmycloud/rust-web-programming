use actix_web::web::ServiceConfig;

pub mod login;
pub mod logout;

pub fn auth_factory(app: &mut ServiceConfig) {}
