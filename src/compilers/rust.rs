use std::env::temp_dir;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::Mutex;

use super::compiler::Compiler;

// Define constants
const WASM_TARGET: &str = "wasm32-wasi";
const RELEASE_BUILD: &str = "release";
const COMPILE_COMMAND: &str = "cargo";
const MAIN_RS_FILE: &str = "src/main.rs";
const WASM_FILE: &str = "app.wasm";

pub struct RustCompiler {
    compile_mutex: Mutex<()>,
}

impl RustCompiler {
    pub fn new() -> Self {
        RustCompiler {
            compile_mutex: Mutex::new(()),
        }
    }
}

impl Compiler for RustCompiler {
    fn lang(&self) -> &'static str {
        "rs"
    }
    fn describe(&self) -> &'static str {
        "rust 2021 edition + rand 0.8.5, release build"
    }
    fn compile(&self, source: &str) -> Result<Vec<u8>, String> {
        let _guard = self.compile_mutex.lock().unwrap_or_else(|e| e.into_inner());

        let binding = temp_dir().join("compilet").join("workspace").join("rs");
        let workspace_dir = binding.to_str().unwrap();

        setup_workspace(Path::new(workspace_dir)).unwrap();

        let mut cargo_command = Command::new(COMPILE_COMMAND);
        cargo_command.current_dir(workspace_dir);
        cargo_command.arg("build");
        cargo_command.arg("--target").arg(WASM_TARGET);
        cargo_command.arg("--release");

        let code_path = Path::new(workspace_dir).join(MAIN_RS_FILE);
        if let Some(parent_path) = code_path.parent() {
            if !parent_path.exists() {
                std::fs::create_dir_all(parent_path).unwrap();
            }
        }
        fs::write(&code_path, source).unwrap();

        match cargo_command.output() {
            Ok(output) if output.status.success() => {
                let wasm_path = Path::new(workspace_dir)
                    .join("target")
                    .join(WASM_TARGET)
                    .join(RELEASE_BUILD)
                    .join(WASM_FILE);
                let wasm = fs::read(&wasm_path).unwrap();

                Ok(wasm)
            }
            Ok(output) => {
                let message = format!(
                    "Compilation failed (code {}):\n{}",
                    output.status.code().unwrap_or(-1),
                    String::from_utf8_lossy(&output.stderr)
                );
                Err(message)
            }
            Err(e) => {
                let message = format!("Error running cargo: {}", e);
                Err(message)
            }
        }
    }
}

pub fn setup_workspace(dir: &Path) -> Result<(), ()> {
    // Create the directory
    fs::create_dir_all(dir).unwrap();

    let cargo_toml_path = dir.join("Cargo.toml");
    if !cargo_toml_path.exists() {
        // Write the Cargo.toml file
        let cargo_toml = format!(
            r#"[package]
name = "app"
version = "1.0.0"
edition = "2021"

[[bin]]
name = "app"
path = "src/main.rs"

[dependencies]
rand = "0.8.5"
"#,
        );
        std::fs::write(&cargo_toml_path, cargo_toml).unwrap();
    }

    Ok(())
}
