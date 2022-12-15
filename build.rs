use std::{fs, path::PathBuf, process::Command};

use wit_component::ComponentEncoder;

fn main() {
    let mut cmd = Command::new("cargo");
    cmd.arg("build")
        .arg("--release")
        .current_dir("./wasi_snapshot_preview1")
        .arg("--target=wasm32-unknown-unknown")
        .env(
            "RUSTFLAGS",
            "-Clink-args=--import-memory -Clink-args=-zstack-size=0",
        )
        .env_remove("CARGO_ENCODED_RUSTFLAGS");
    let status = cmd.status().unwrap();
    assert!(status.success());

    let wasi_adapter =
        PathBuf::from("./wasi_snapshot_preview1/target/wasm32-unknown-unknown/release/wasi_snapshot_preview1.wasm");
    println!("wasi adapter: {:?}", &wasi_adapter);
    let wasi_adapter = std::fs::read(&wasi_adapter).unwrap();

    let mut cmd = Command::new("cargo");
    cmd.arg("build")
        .current_dir("./guest")
        .arg("--target=wasm32-wasi")
        .env("CARGO_PROFILE_DEV_DEBUG", "1")
        .env("RUSTFLAGS", "-Clink-args=--export-table")
        .env_remove("CARGO_ENCODED_RUSTFLAGS");
    let status = cmd.status().unwrap();
    assert!(status.success());

    let file = PathBuf::from("./guest/target/wasm32-wasi/debug/guest.wasm");
    let stem = file.file_stem().unwrap().to_str().unwrap().to_string();
    println!("stem: {:?}", &stem);

    // Translate the canonical ABI module into a component.
    let module = fs::read(&file).expect("failed to read wasm file");
    let component = ComponentEncoder::default()
        .module(module.as_slice())
        .expect("pull custom sections from module")
        .validate(true)
        .adapter("wasi_snapshot_preview1", &wasi_adapter)
        .expect("adapter failed to get loaded")
        .encode()
        .expect(&format!(
            "module {:?} can be translated to a component",
            file
        ));
    let component_path = format!("./target/{}.component.wasm", stem);
    fs::write(&component_path, component).expect("write component to disk");

    println!("cargo:rerun-if-changed=./guest/Cargo.toml");
    println!("cargo:rerun-if-changed=./wasi_snapshot_preview1");
}
