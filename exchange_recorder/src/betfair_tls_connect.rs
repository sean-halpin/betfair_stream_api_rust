use crate::app_config_load::AppConfig;
use native_tls::TlsConnector;
use native_tls::TlsStream;
use std::io::Write;
use std::net::TcpStream;

pub fn connect_betfair_tls_stream(cfg: &AppConfig) -> Result<TlsStream<TcpStream>, String> {
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
