use crate::metrics::metrics_statics::INCOMING_MESSAGES;
use crate::AppConfig;
use mongodb::{bson::doc, Client};
use native_tls::TlsConnector;
use native_tls::TlsStream;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpStream;

fn connect_betfair_tls_stream(cfg: &AppConfig) -> Result<TlsStream<TcpStream>, String> {
    println!("TLS connect starting");

    let auth_msg = format!(
        "{{\"op\": \"authentication\",\"id\":1, \"appKey\": \"{}\", \"session\": \"{}\"}}\r\n",
        &cfg.app_key, &cfg.ssoid
    );
    let sub_msg = format!(
        "{{\"op\":\"marketSubscription\",\"id\":1,\"marketFilter\":{{\"marketIds\":[\"{}\"]}}}}\r\n",
        &cfg.market_id
    );
    println!("{}", auth_msg);
    println!("{}", sub_msg);

    let connector = TlsConnector::new().unwrap();

    let tcp_stream = match TcpStream::connect(&cfg.stream_api_endpoint) {
        Ok(stream) => stream,
        Err(e) => return Err(e.to_string()),
    };
    let mut tls_stream = match connector.connect(&cfg.stream_api_host, tcp_stream) {
        Ok(stream) => stream,
        Err(e) => return Err(e.to_string()),
    };

    tls_stream.write_all(auth_msg.as_bytes()).unwrap();
    tls_stream.write_all(sub_msg.as_bytes()).unwrap();

    return Ok(tls_stream);
}

pub async fn subscribe(cfg: &AppConfig) {
    let market_id = &cfg.market_id;
    let stream = connect_betfair_tls_stream(&cfg).unwrap();

    let client = Client::with_uri_str(&cfg.mongo_conn).await.unwrap();
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
