pub mod cluster;
pub mod wasm;

pub struct QuantiiService {}

impl QuantiiService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run_service(&self) -> bool {
        true
    }
}

// ASSUME QUANTII DESKTOP API
pub fn start_quantii_desktop() {}
