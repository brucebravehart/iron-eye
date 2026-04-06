use dotenvy::dotenv;
use std::env;

pub struct RuntimeConfig {
    pub trading_mode: String,
    pub broker_name: String,
    pub signal_file: String,
}

impl RuntimeConfig {
    pub fn load() -> Self {
        dotenv().ok();

        Self {
            trading_mode: env::var("TRADING_MODE").unwrap_or_else(|_| "paper".to_string()),
            broker_name: env::var("BROKER_NAME").unwrap_or_else(|_| "paper".to_string()),
            signal_file: env::var("SIGNAL_FILE")
                .unwrap_or_else(|_| "signals/latest.json".to_string()),
        }
    }
}
