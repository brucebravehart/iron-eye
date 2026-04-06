mod brokers;
mod config;
mod models;

use anyhow::Result;
use brokers::base::BrokerClient;
use brokers::paper::PaperBroker;
use config::RuntimeConfig;
use models::{TradeSignal, TradingDecision};
use std::fs;
use std::path::Path;

fn load_signals(path: &str) -> Result<Vec<TradeSignal>> {
    if !Path::new(path).exists() {
        return Ok(Vec::new());
    }

    let payload = fs::read_to_string(path)?;
    let signals: Vec<TradeSignal> = serde_json::from_str(&payload)?;
    Ok(signals)
}

fn apply_model_decision(signal: &TradeSignal) -> TradingDecision {
    // Placeholder gate until a real ONNX model is added to models/policy.onnx.
    // Once imported by burn-import, call the generated model here.
    let approved = signal.quantity > 0.0;
    TradingDecision {
        approved,
        confidence: if approved { 0.65 } else { 0.0 },
    }
}

fn main() -> Result<()> {
    let cfg = RuntimeConfig::load();
    let broker = PaperBroker::new();
    let signals = load_signals(&cfg.signal_file)?;

    for signal in signals {
        let decision = apply_model_decision(&signal);
        if !decision.approved {
            continue;
        }

        let result = broker.place_order(&signal)?;
        println!("{}", serde_json::to_string(&result)?);
    }

    Ok(())
}
