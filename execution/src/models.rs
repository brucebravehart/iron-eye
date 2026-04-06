use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TradeSignal {
    pub symbol: String,
    pub side: OrderSide,
    pub quantity: f64,
    pub limit_price: Option<f64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrderResult {
    pub broker_order_id: String,
    pub status: String,
    pub message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TradingDecision {
    pub approved: bool,
    pub confidence: f32,
}
