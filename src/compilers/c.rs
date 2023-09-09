use super::compiler::Compiler;
use home::home_dir;
use std::env::current_dir;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;

const COMPILE_COMMAND: &str = "clang";
const SOURCE_FILE: &str = "main.c";
const WASM_FILE: &str = "app.wasm";
const MAX_ERRORS: &str = "10";

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
    fn lang(&self) -> String {
        String::from("c")
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

        let sysroot = setup_workspace(Path::new(workspace))?;

        let source_path = Path::new(workspace).join(SOURCE_FILE);
        fs::write(source_path, source).unwrap();

        let wasm_path = Path::new(workspace).join(WASM_FILE);

        let mut compile_command = Command::new(COMPILE_COMMAND);
        compile_command.current_dir(workspace).args([
            "-O3",
            "-o",
            WASM_FILE,
            "-target",
            "wasm32-wasi",
            "--sysroot",
            sysroot.to_str().unwrap_or_default(),
            "-ferror-limit",
            MAX_ERRORS,
            SOURCE_FILE,
        ]);

        let output = compile_command
            .output()
            .map_err(|e| format!("Error running {}: {}", COMPILE_COMMAND, e))?;

        if output.status.success() {
            let wasm = fs::read(&wasm_path).map_err(|e| {
                format!(
                    "Error reading compiled output file {}: {}",
                    wasm_path.display(),
                    e
                )
            })?;
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

pub fn setup_workspace(_dir: &Path) -> Result<PathBuf, String> {
    let sysroot_path = find_sysroot().unwrap_or_else(|_| {
        home_dir()
            .unwrap()
            .join(".compilet")
            .join("stdlib")
            .join("wasi-sysroot")
    });

    if !sysroot_path.exists() {
        return Err(format!(
            "WASI sysroot not found at {}",
            sysroot_path.to_str().unwrap()
        ));
    }

    Ok(fs::canonicalize(sysroot_path).unwrap())
}

fn find_sysroot() -> Result<PathBuf, String> {
    let mut base = match current_dir() {
        Ok(base_dir) => base_dir,
        Err(e) => return Err(format!("Failed to get current directory: {}", e)),
    };
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
