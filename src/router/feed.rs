use crate::handler::feed::feed;
use axum::routing::post;
use axum::Router;

pub fn feed_routers() -> Router {
    Router::new().route("/feeds", post(feed))
}
