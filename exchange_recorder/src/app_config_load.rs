use envconfig::Envconfig;

#[derive(Envconfig, Clone)]
pub struct AppConfig {
    #[envconfig(from = "SSOID", default = "XXXX")]
    pub ssoid: String,
    #[envconfig(from = "APP_KEY", default = "XXXX")]
    pub app_key: String,
    #[envconfig(from = "STREAM_API_ENDPOINT", default = "stream-api.betfair.com:443")]
    pub stream_api_endpoint: String,
    #[envconfig(from = "STREAM_API_HOST", default = "stream-api.betfair.com")]
    pub stream_api_host: String,
    #[envconfig(from = "MARKET_ID", default = "1.142069XXX")]
    pub market_id: String,
}
