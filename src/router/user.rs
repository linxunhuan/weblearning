use crate::handler::user::{create_user, login};
use axum::routing::post;
use axum::Router;

pub fn user_routers() -> Router {
    Router::new()
        .route("/users", post(create_user))
        .route("/login", post(login))
}
