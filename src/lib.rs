use crate::config::config_init;
use crate::model::db_conn_init;
use crate::router::start_route;

pub mod config;
pub mod entities;
pub mod handler;
pub mod middleware;
pub mod model;
pub mod router;
pub mod service;
pub mod utils;

pub async fn run() {
    //初始化
    config_init().await;

    //链接数据库
    db_conn_init().await;

    //开启路由
    start_route().await;
}
