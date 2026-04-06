use anyhow::Result;

use crate::brokers::base::BrokerClient;
use crate::models::{OrderResult, TradeSignal};

pub struct PaperBroker;

impl PaperBroker {
    pub fn new() -> Self {
        Self
    }
}

impl BrokerClient for PaperBroker {
    fn place_order(&self, signal: &TradeSignal) -> Result<OrderResult> {
        Ok(OrderResult {
            broker_order_id: format!("paper-{}-{}", signal.symbol, chrono_like_now()),
            status: "accepted".to_string(),
            message: Some(format!("Simulated order for {}", signal.symbol)),
        })
    }
}

fn chrono_like_now() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or_default()
}
