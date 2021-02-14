use crate::config::app_config::AppConfig;
use crate::metrics::metrics_statics::INCOMING_MESSAGES;
use crate::metrics::metrics_statics::PROM_REGISTRY;
use prometheus::{Encoder, TextEncoder};

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

pub async fn start_web_server(_cfg: &AppConfig) {
    register_custom_metrics();
    println!("Starting Prometheus Metrics Endpoint: http://localhost:8000/metrics");
    rocket::ignite()
        .mount("/metrics", routes![metrics])
        .launch();
}
