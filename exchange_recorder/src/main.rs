#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
mod config;
mod exchange_recorder;
mod metrics;
use crate::exchange_recorder::exchange_subscribe_recorder::subscribe;
use crate::metrics::metrics_server::start_web_server;

#[tokio::main]
async fn main() {
    let config = config::app_config::load().unwrap();

    let moved_config = config.clone();
    tokio::spawn(async move { start_web_server(&moved_config).await });
    subscribe(&config).await;
}
