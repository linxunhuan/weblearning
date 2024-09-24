use crate::handler::follower::{follow, unfollow};
use axum::routing::{delete, post};
use axum::Router;

pub fn follower_routers() -> Router {
    Router::new()
        .route("/followers", post(follow))
        .route("/followers/:uid", delete(unfollow))
}
