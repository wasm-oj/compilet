use home::home_dir;
use rand::{distributions::Alphanumeric, Rng};
use std::env::{current_dir, temp_dir};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;

use super::compiler::Compiler;

// Define constants
const COMPILE_COMMAND: &str = "clang";
const SOURCE_FILE: &str = "main.c";
const WASM_FILE: &str = "app.wasm";

pub struct CCompiler {
    compile_mutex: Mutex<()>,
}

impl CCompiler {
    pub fn new() -> Self {
        CCompiler {
            compile_mutex: Mutex::new(()),
        }
    }
}

impl Compiler for CCompiler {
    fn lang(&self) -> &'static str {
        "c"
    }
    fn describe(&self) -> &'static str {
        "clang 16, level 3 optimizations"
    }
    fn compile(&self, source: &str) -> Result<Vec<u8>, String> {
        let _guard = self.compile_mutex.lock().unwrap_or_else(|e| e.into_inner());

        let rand_string: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();

        let binding = temp_dir()
            .join("compilet")
            .join("workspace")
            .join("c")
            .join(rand_string);
        let workspace_dir = binding.to_str().unwrap();

        let sysroot = setup_workspace(Path::new(workspace_dir))?;

        let source_path = Path::new(workspace_dir).join(SOURCE_FILE);
        fs::write(source_path, source).unwrap();

        let wasm_path = Path::new(workspace_dir).join(WASM_FILE);

        let mut compile_command = Command::new(COMPILE_COMMAND);
        compile_command.current_dir(workspace_dir);
        compile_command.arg("-O3");
        compile_command.arg("-o").arg(WASM_FILE);
        compile_command.arg("-target").arg("wasm32-wasi");
        compile_command.arg("--sysroot").arg(sysroot);
        compile_command.arg(SOURCE_FILE);

        match compile_command.output() {
            Ok(output) if output.status.success() => {
                let wasm = fs::read(wasm_path).unwrap();
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

pub fn setup_workspace(dir: &Path) -> Result<PathBuf, String> {
    // Create the directory
    fs::create_dir_all(dir).unwrap();

    let sysroot_path = find_sysroot();
    let sysroot_path = match sysroot_path {
        Ok(sysroot_path) => sysroot_path,
        Err(_) => home_dir()
            .unwrap()
            .join(".compilet")
            .join("stdlib")
            .join("wasi-sysroot"),
    };

    if !sysroot_path.exists() {
        return Err(format!(
            "WASI sysroot not found at {}",
            sysroot_path.to_str().unwrap()
        ));
    }

    Ok(fs::canonicalize(sysroot_path).unwrap())
}

fn find_sysroot() -> Result<PathBuf, String> {
    let mut base = current_dir().unwrap();
    let mut path = base.join("stdlib/wasi-sysroot");
    while !path.exists() {
        base = base.parent().unwrap().to_path_buf();
        path = base.join("stdlib/wasi-sysroot");
        if base == PathBuf::from("/") {
            return Err("WASI sysroot not found".to_string());
        }
    }
    Ok(path)
}
