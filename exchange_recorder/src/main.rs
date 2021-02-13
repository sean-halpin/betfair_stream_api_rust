#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
mod app_config_load;
mod betfair_tls_connect;
use crate::app_config_load::AppConfig;
use envconfig::Envconfig;
use lazy_static::lazy_static;
use mongodb::{bson::doc, Client};
use prometheus::{Encoder, IntCounter, Registry, TextEncoder};
use std::io::BufRead;
use std::io::BufReader;

lazy_static! {
    pub static ref INCOMING_MESSAGES: IntCounter =
        IntCounter::new("incoming_messages", "Incoming Messages").expect("metric can be created");
    pub static ref PROM_REGISTRY: Registry = Registry::new();
}

fn register_custom_metrics() {
    PROM_REGISTRY
        .register(Box::new(INCOMING_MESSAGES.clone()))
        .expect("collector can be registered");
}

#[get("/")]
fn metrics() -> String {
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let gathered = PROM_REGISTRY.gather();
    encoder.encode(&gathered, &mut buffer).unwrap();

    return String::from_utf8(buffer).unwrap();
}

async fn start_web_server(_cfg: &AppConfig) {
    println!("Starting Prometheus Metrics Endpoint: http://localhost:8000/metrics");
    rocket::ignite()
        .mount("/metrics", routes![metrics])
        .launch();
}

async fn subscribe_to_betfair_exchange(cfg: &app_config_load::AppConfig) {
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

    register_custom_metrics();
    let moved_config = config.clone();
    tokio::spawn(async move { start_web_server(&moved_config).await });
    subscribe_to_betfair_exchange(&config).await;
}
