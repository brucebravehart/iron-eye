use std::path::Path;

fn main() {
    let onnx_path = "models/policy.onnx";
    println!("cargo:rerun-if-changed={onnx_path}");

    if Path::new(onnx_path).exists() {
        burn_import::onnx::ModelGen::new()
            .input(onnx_path)
            .out_dir("src/model")
            .run_from_script();
    }
}
