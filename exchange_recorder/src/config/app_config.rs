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
    #[envconfig(
        from = "MONGO_CONN",
        default = "mongodb://root:password123@0.0.0.0:27017/"
    )]
    pub mongo_conn: String,
}

pub fn load() -> Result<AppConfig, String> {
    return match AppConfig::init_from_env() {
        Ok(cfg) => Ok(cfg),
        Err(_) => Err("Could not Load App Config".to_string()),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn default_config_used() {
        env::remove_var("SSOID");
        env::remove_var("APP_KEY");
        env::remove_var("STREAM_API_ENDPOINT");
        env::remove_var("STREAM_API_HOST");
        env::remove_var("MARKET_ID");
        env::remove_var("MONGO_CONN");

        let app_config = load().unwrap();
        
        assert_eq!(app_config.ssoid, "XXXX");
        assert_eq!(app_config.app_key, "XXXX");
        assert_eq!(app_config.stream_api_endpoint, "stream-api.betfair.com:443");
        assert_eq!(app_config.stream_api_host, "stream-api.betfair.com");
        assert_eq!(app_config.market_id, "1.142069XXX");
        assert_eq!(
            app_config.mongo_conn,
            "mongodb://root:password123@0.0.0.0:27017/"
        );
    }

    #[test]
    fn env_var_config_used() {
        env::set_var("SSOID", "TEST_VALUE_1");
        env::set_var("APP_KEY", "TEST_VALUE_2");
        env::set_var("STREAM_API_ENDPOINT", "TEST_VALUE_3");
        env::set_var("STREAM_API_HOST", "TEST_VALUE_4");
        env::set_var("MARKET_ID", "TEST_VALUE_5");
        env::set_var("MONGO_CONN", "TEST_VALUE_6");

        let app_config = load().unwrap();

        assert_eq!(app_config.ssoid, "TEST_VALUE_1");
        assert_eq!(app_config.app_key, "TEST_VALUE_2");
        assert_eq!(app_config.stream_api_endpoint, "TEST_VALUE_3");
        assert_eq!(app_config.stream_api_host, "TEST_VALUE_4");
        assert_eq!(app_config.market_id, "TEST_VALUE_5");
        assert_eq!(app_config.mongo_conn, "TEST_VALUE_6");
    }
}
