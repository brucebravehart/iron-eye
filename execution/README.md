# execution

Automated order execution workspace in Rust.

## Responsibilities

- Read approved signals from research outputs.
- Run model-assisted risk checks and position sizing.
- Send orders to broker APIs.
- Track order state and execution logs.

## Important

- Keep this project in paper mode until risk controls are verified.
- Implement broker-specific retries, idempotency, and error handling.

## Burn + ONNX

- Put your ONNX model at `models/policy.onnx`.
- `build.rs` will run `burn-import` and generate model code into `src/model/`.
- Integrate the generated model in `src/main.rs` in `apply_model_decision`.

## Local Run (Paper Mode)

1. Install stable Rust toolchain (`rustup default stable-x86_64-pc-windows-msvc`).
2. Copy `.env.example` to `.env`.
3. Copy `signals/latest.example.json` to `signals/latest.json`.
4. From this folder, run `cargo run`.

## Troubleshooting

- If you see `dlltool.exe` missing during build, switch to MSVC toolchain and install Visual Studio Build Tools (C++ workload).
