use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::Mutex;

use super::compiler::Compiler;

// Define constants
const COMPILE_COMMAND: &str = "clang++";
const WORKSPACE_DIR: &str = "workspace/cpp";
const SOURCE_FILE: &str = "main.cpp";
const WASM_FILE: &str = "app.wasm";

pub struct CppCompiler {
    compile_mutex: Mutex<()>,
}

impl CppCompiler {
    pub fn new() -> Self {
        CppCompiler {
            compile_mutex: Mutex::new(()),
        }
    }
}

impl Compiler for CppCompiler {
    fn compile(&self, source: &str) -> Result<Vec<u8>, String> {
        let _guard = self.compile_mutex.lock().unwrap_or_else(|e| e.into_inner());

        setup_workspace(Path::new(WORKSPACE_DIR)).unwrap();

        let source_path = Path::new(WORKSPACE_DIR).join(SOURCE_FILE);
        fs::write(&source_path, source).unwrap();

        let wasm_path = Path::new(WORKSPACE_DIR).join(WASM_FILE);

        let mut compile_command = Command::new(COMPILE_COMMAND);
        compile_command.current_dir(WORKSPACE_DIR);
        compile_command.arg("-O3");
        compile_command.arg("-o").arg(WASM_FILE);
        compile_command.arg("-target").arg("wasm32-wasi");
        compile_command.arg("-fno-exceptions");
        compile_command
            .arg("--sysroot")
            .arg("../../stdlib/wasi-sysroot");
        compile_command.arg(SOURCE_FILE);

        match compile_command.output() {
            Ok(output) if output.status.success() => {
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
                let message = format!("Error running {}: {}", COMPILE_COMMAND, e);
                Err(message)
            }
        }
    }
}

pub fn setup_workspace(dir: &Path) -> Result<(), ()> {
    // Create the directory
    fs::create_dir_all(dir).unwrap();

    Ok(())
}
