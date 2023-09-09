use rocket::serde::{Deserialize, Serialize};
use std::path::PathBuf;

// Define a struct to represent incoming code submissions
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CodeSubmission {
    pub lang: String,
    pub code: String,
}

// Define a struct to represent the result of a compilation
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CompileResult {
    pub success: bool,
    pub message: String,
    pub hash: Option<String>,
    pub wasm: Option<String>,
}

// Define a Rocket state to hold a cache of compiled WebAssembly modules
pub struct WasmCache {
    pub dir: PathBuf,
}
