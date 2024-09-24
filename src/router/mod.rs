pub mod feed;
pub mod follower;
pub mod post;
pub mod user;

use crate::middleware::jwt::Auth;
use crate::router::feed::feed_routers;
use crate::router::follower::follower_routers;
use crate::router::post::post_routers;
use crate::router::user::user_routers;
use axum::{middleware, Router};

pub async fn start_route() {
    //用户路由
    let user_routers = user_routers();
    let follower_routers = follower_routers();
    let post_routers = post_routers();
    let feed_routers = feed_routers();

    let app = Router::new()
        .merge(feed_routers)
        .merge(post_routers)
        .merge(follower_routers)
        .layer(middleware::from_extractor::<Auth>()) //添加一个中间件层，用于从请求中提取认证信息
        .merge(user_routers);

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
