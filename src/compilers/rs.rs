use super::compiler::Compiler;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::Mutex;

const WASM_TARGET: &str = "wasm32-wasi";
const RELEASE_BUILD: &str = "release";
const COMPILE_COMMAND: &str = "cargo";
const MAIN_RS_FILE: &str = "src/main.rs";
const WASM_FILE: &str = "app.wasm";

pub struct RsCompiler {
    compile_mutex: Mutex<()>,
}

impl RsCompiler {
    pub fn new() -> Self {
        RsCompiler {
            compile_mutex: Mutex::new(()),
        }
    }
}

impl Compiler for RsCompiler {
    fn lang(&self) -> String {
        String::from("rs")
    }

    fn describe(&self) -> String {
        let version_output = Command::new(COMPILE_COMMAND).arg("--version").output();
        let version = match version_output {
            Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
            Err(_) => String::from("unknown"),
        };

        format!("{}", version.trim())
    }

    fn compile(&self, source: &str, workspace: &str) -> Result<Vec<u8>, String> {
        let _guard = self.compile_mutex.lock().unwrap_or_else(|e| e.into_inner());

        setup_workspace(Path::new(workspace)).unwrap();

        let mut cargo_command = Command::new(COMPILE_COMMAND);
        cargo_command
            .current_dir(workspace)
            .args(["build", "--target", WASM_TARGET, "--release"]);

        let code_path = Path::new(workspace).join(MAIN_RS_FILE);
        if let Some(parent_path) = code_path.parent() {
            if !parent_path.exists() {
                std::fs::create_dir_all(parent_path).unwrap();
            }
        }
        fs::write(&code_path, source).unwrap();

        let output = cargo_command
            .output()
            .map_err(|e| format!("Error running {}: {}", COMPILE_COMMAND, e))?;

        if output.status.success() {
            let wasm_path = Path::new(workspace)
                .join("target")
                .join(WASM_TARGET)
                .join(RELEASE_BUILD)
                .join(WASM_FILE);
            let wasm = fs::read(wasm_path).unwrap();

            Ok(wasm)
        } else {
            let message = format!(
                "Compilation failed (code {}):\n{}",
                output.status.code().unwrap_or(-1),
                String::from_utf8_lossy(&output.stderr)
            );
            Err(message)
        }
    }
}

pub fn setup_workspace(dir: &Path) -> Result<(), ()> {
    let cargo_toml_path = dir.join("Cargo.toml");
    if !cargo_toml_path.exists() {
        // Write the Cargo.toml file
        let cargo_toml = r#"[package]
name = "app"
version = "1.0.0"
edition = "2021"

[[bin]]
name = "app"
path = "src/main.rs"

[dependencies]
rand = "0.8.5"
"#;
        std::fs::write(&cargo_toml_path, cargo_toml).unwrap();
    }

    Ok(())
}
