#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
mod betfair_tls_connect;
mod config;
mod metrics;
use config::app_config_load::AppConfig;
use envconfig::Envconfig;
use metrics::metrics_server::start_web_server;
use metrics::metrics_statics::INCOMING_MESSAGES;
use mongodb::{bson::doc, Client};
use std::io::BufRead;
use std::io::BufReader;

async fn subscribe_to_betfair_exchange(cfg: &AppConfig) {
    let market_id = &cfg.market_id;
    let stream = betfair_tls_connect::connect_betfair_tls_stream(&cfg).unwrap();

    let client = Client::with_uri_str("mongodb://root:password123@0.0.0.0:27017/")
        .await
        .unwrap();
    let db = client.database("betfair_exchange_db");
    let coll = db.collection(&market_id);

    let mut stream_reader = BufReader::new(stream);
    let mut buf = String::new();
    while stream_reader.read_line(&mut buf).unwrap_or(0) > 0 {
        INCOMING_MESSAGES.inc();
        println!("{}", &buf);
        let result = match coll.insert_one(doc! { "payload": &buf}, None).await {
            Ok(_) => Ok("Inserted a document into MongoDB"),
            Err(e) => Err(e),
        };
        println!("{:#?}", result);
        buf = "".to_string();
    }
}

#[tokio::main]
async fn main() {
    let config = match AppConfig::init_from_env() {
        Ok(cfg) => cfg,
        Err(_) => panic! {"Could not Load App Config"},
    };

    let moved_config = config.clone();
    tokio::spawn(async move { start_web_server(&moved_config).await });
    subscribe_to_betfair_exchange(&config).await;
}
