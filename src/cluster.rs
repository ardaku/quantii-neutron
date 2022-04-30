// --------------
// CLUSTERS
// --------------

use super::wasm::WasmContainer;

// good ref https://developers.redhat.com/blog/2020/05/11/top-10-must-know-kubernetes-design-patterns

struct Cluster {
    containers: Vec<WasmContainer>,
}

// Randomised heuristics

// given n users who want to access the database at once
// choose a random one with a simple randomised hueristic
fn generic_database_access() {}
