use actix_web::web::{ServiceConfig, post, scope};
use auth_dal::users::SqlxPostGresDescriptor;

pub mod create;

pub fn users_factory(app: &mut ServiceConfig) {
    app.service(scope("/api/v1/users").route(
        "create",
        post().to(create::create::<SqlxPostGresDescriptor>),
    ));
}
