use actix_web::web;
//use crate::handlers::user_handler::{create_user, delete_user, get_user, get_users, update_user};
use crate::handlers::user_handler::{create_user, delete_user, get_user, get_users};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/users")
            .route("", web::post().to(create_user))
            .route("/{id}", web::get().to(get_user))
            .route("", web::get().to(get_users))
            //.route("", web::put().to(update_user))
            .route("/{id}", web::delete().to(delete_user)),
    );
}