#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use lazy_static::lazy_static;
use mongodb::{bson::doc, Client};
use native_tls::TlsConnector;
use prometheus::{Encoder, IntCounter, Registry, TextEncoder};
use std::env;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpStream;

lazy_static! {
    pub static ref INCOMING_MESSAGES: IntCounter =
        IntCounter::new("incoming_messages", "Incoming Messages").expect("metric can be created");
    pub static ref REGISTRY: Registry = Registry::new();
}

fn register_custom_metrics() {
    REGISTRY
        .register(Box::new(INCOMING_MESSAGES.clone()))
        .expect("collector can be registered");
}

#[get("/")]
fn metrics() -> String {
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let gathered = REGISTRY.gather();
    encoder.encode(&gathered, &mut buffer).unwrap();

    return String::from_utf8(buffer).unwrap();
}

async fn start_web_server() {
    println!("Starting Prometheus Metrics Endpoint: http://localhost:8000/metrics");
    rocket::ignite()
        .mount("/metrics", routes![metrics])
        .launch();
}

async fn subscribe_to_betfair_exchange() {
    println!("Betfair Exchange Stream Recorder Started");
    let stream_api_endpoint = "stream-api.betfair.com:443".to_owned();
    let stream_api_host = "stream-api.betfair.com".to_owned();
    let ssoid = env::var("SSOID").unwrap_or("none".to_string());
    let app_key = env::var("APP_KEY").unwrap_or("none".to_string());
    let market_id = env::var("MARKET_ID").unwrap_or("none".to_string());

    let auth_msg = format!(
        "{{\"op\": \"authentication\",\"id\":1, \"appKey\": \"{}\", \"session\": \"{}\"}}\r\n",
        app_key, ssoid
    );
    let sub_msg = format!(
        "{{\"op\":\"marketSubscription\",\"id\":1,\"marketFilter\":{{\"marketIds\":[\"{}\"]}}}}\r\n",
        market_id
    );
    println!("{}", auth_msg);
    println!("{}", sub_msg);

    let connector = TlsConnector::new().unwrap();

    let stream = TcpStream::connect(stream_api_endpoint).unwrap();
    let mut stream = connector.connect(&stream_api_host, stream).unwrap();

    stream.write_all(auth_msg.as_bytes()).unwrap();
    stream.write_all(sub_msg.as_bytes()).unwrap();

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
    register_custom_metrics();
    tokio::spawn(async move { start_web_server().await });
    subscribe_to_betfair_exchange().await
}
