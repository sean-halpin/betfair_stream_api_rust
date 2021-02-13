#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
mod config;
mod exchange_recorder;
mod metrics;
use crate::config::app_config_load::AppConfig;
use crate::exchange_recorder::exchange_subscribe_recorder::subscribe;
use crate::metrics::metrics_server::start_web_server;
use envconfig::Envconfig;

#[tokio::main]
async fn main() {
    let config = match AppConfig::init_from_env() {
        Ok(cfg) => cfg,
        Err(_) => panic! {"Could not Load App Config"},
    };

    let moved_config = config.clone();
    tokio::spawn(async move { start_web_server(&moved_config).await });
    subscribe(&config).await;
}
