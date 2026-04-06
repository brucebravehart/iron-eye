# ONNX Models

Place execution policy ONNX files in this folder.

Expected default path:

- `models/policy.onnx`

During `cargo build` or `cargo run`, `build.rs` will detect this file and use `burn-import` to generate Rust model code in `src/model/`.
