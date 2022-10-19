use std::env::{self, VarError};

pub struct Config {
    pub config_path: String,
    pub journal_port: String,
    pub scrape_interval: String,
    pub vector_url: Result<String, VarError>,
    pub use_ipv6: bool,
    pub log_path: String,
}

pub fn new_config() -> Config {
    let config_path = env::var("CONFIG_PATH").unwrap_or("config.json".to_string());
    let default_port = env::var("JOURNAL_PORT").unwrap_or("5000".to_string());
    let scrape_interval = env::var("SCRAPE_INTERVAL").unwrap_or("2".to_string());
    let vector_url = env::var("VECTOR_INSTANCE_IP");
    let use_ipv6 = env::var("USE_IPV6").unwrap_or("false".to_string()) == "true";
    let log_path = env::var("LOG_PATH").unwrap_or("/logs".to_string());

    Config {
        config_path,
        journal_port: default_port,
        scrape_interval,
        vector_url,
        use_ipv6,
        log_path,
    }
}
