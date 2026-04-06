# iron-eye

**This Project is still Work-In-Progress**
Monorepo for automated asset discovery research and broker API execution.

## Purpose

This project has two tracks:

- **research**: Discover, score, and shortlist candidate assets.
- **execution**: Connect to broker APIs and place orders based on approved signals.

## Repository Structure

- `research/`: Discovery and analysis tooling.
- `execution/`: Trading engine and broker adapters.

## Quick Start

1. Create and activate a Python virtual environment for research.
2. Install the research package from its folder:
   - `cd research && pip install -e .`
3. Ensure Rust toolchain is installed for execution:
   - `rustup default stable-x86_64-pc-windows-msvc`
   - If prompted, install Visual Studio Build Tools with C++ workload.
4. Copy and edit execution environment settings:
   - `copy execution/.env.example execution/.env`
5. (Optional) Seed sample signals for a paper run:
   - `copy execution/signals/latest.example.json execution/signals/latest.json`
6. Run the execution engine from inside `execution/`:
   - `cargo run`
7. Implement your broker adapters and risk controls before enabling live trading.

## Safety Notes

- Keep API keys in `.env`, never in source control.
- Start with a paper-trading broker or sandbox API.
- Add strict risk limits before sending real orders.
- Ensure your automation complies with broker and market rules in your jurisdiction.
