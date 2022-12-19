use std::{path::PathBuf, process::Command};

fn main() {
    let mut cmd = Command::new("curl");
    cmd.arg("-LO")
        .arg("https://github.com/bytecodealliance/preview2-prototyping/releases/download/latest/wasi_snapshot_preview1.wasm");
    let status = cmd.status().unwrap();
    assert!(status.success());

    let wasi_adapter = PathBuf::from("./wasi_snapshot_preview1.wasm");
    println!("wasi adapter: {:?}", &wasi_adapter);

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
    // let component = ComponentEncoder::default()
    //     .module(module.as_slice())
    //     .expect("pull custom sections from module")
    //     .validate(true)
    //     .adapter("wasi_snapshot_preview1", &wasi_adapter)
    //     .expect("adapter failed to get loaded")
    //     .encode()
    //     .unwrap_or_else(|e| panic!("{}", e));
    // let component_path = format!("./target/{stem}.component.wasm");
    // fs::write(component_path, component).expect("write component to disk");

    // check if wasm-tools is installed
    let mut cmd = Command::new("wasm-tools");
    cmd.arg("--version");
    let status = cmd.status().unwrap();
    assert!(status.success());

    // Translate the canonical ABI module into a component.
    let mut cmd = Command::new("wasm-tools");
    cmd.arg("component")
        .arg("new")
        .arg(&file)
        .arg("--adapt")
        .arg(&wasi_adapter)
        .arg("-o")
        .arg(format!("./target/{stem}.component.wasm"));
    let status = cmd.status().unwrap();
    assert!(status.success());

    println!("cargo:rerun-if-changed=./guest/Cargo.toml");
    println!("cargo:rerun-if-changed=./wasi_snapshot_preview1.wasm");
}
