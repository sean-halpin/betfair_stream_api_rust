use lazy_static::lazy_static;
use prometheus::{IntCounter, Registry};

lazy_static! {
    pub static ref INCOMING_MESSAGES: IntCounter =
        IntCounter::new("incoming_messages", "Incoming Messages").expect("metric can be created");
    pub static ref PROM_REGISTRY: Registry = Registry::new();
}
