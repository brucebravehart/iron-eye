use anyhow::Result;

use crate::models::{OrderResult, TradeSignal};

pub trait BrokerClient {
    fn place_order(&self, signal: &TradeSignal) -> Result<OrderResult>;
}
