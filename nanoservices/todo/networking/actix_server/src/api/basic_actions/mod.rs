pub mod create;
pub mod delete;
pub mod get;
pub mod update;

use actix_web::web::{ServiceConfig, delete, get, post, put, scope};
use todo_dal::todo_items::descriptors::SqlxPostGresDescriptor;

pub fn basic_actions_factory(app: &mut ServiceConfig) {
    app.service(
        scope("/api/v1")
            .route("get/all", get().to(get::get_all::<SqlxPostGresDescriptor>))
            .route("get/{name}", get().to(get::get_by_name))
            .route(
                "create",
                post().to(create::create::<SqlxPostGresDescriptor>),
            )
            .route(
                "delete/{name}",
                delete().to(delete::delete_by_name::<SqlxPostGresDescriptor>),
            )
            .route("update", put().to(update::update::<SqlxPostGresDescriptor>)),
    );
}
