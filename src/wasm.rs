// ------------------
// WASM COMPATIBILITY
// ------------------

// WASM Layer for wasm32/64 executables

pub struct WasmContainer;

pub struct WasmProcess;

// ------------------
// CONTAINER FUNCTIONALITY
// ------------------

// KRUSTLET

pub struct Kubelet {}

pub fn start_krustlet() {
    Command::new("krustlet")
        .spawn()
        .expect("Could not start krustlet");
}

pub fn create_kubelet() -> Kubelet {
    Kubelet {}
}

// OCI

use std::process::Command;

// if runc not already started, start it
pub fn start_runc_service() {}

pub fn spawn_runc_container() -> WasmContainer {
    WasmContainer {}
}
