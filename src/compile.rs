use crate::compilers;
use crate::config::*;
use crate::jwt;
use base64::{engine::general_purpose, Engine as _};
use compilers::get_compiler_for_language;
use rocket::serde::{
    json::{Error, Json},
    Deserialize, Serialize,
};
use rocket::State;
use sha256::digest;
use std::fs;
use std::path::PathBuf;

// Define a struct to represent incoming code submissions
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CodeSubmission {
    lang: String,
    code: String,
}

// Define a struct to represent the result of a compilation
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CompileResult {
    success: bool,
    message: String,
    hash: Option<String>,
    wasm: Option<String>,
}

// Define a Rocket state to hold a cache of compiled WebAssembly modules
pub struct WasmCache {
    pub dir: PathBuf,
}

// Define a Rocket route to handle incoming code submissions
#[post("/compile", format = "json", data = "<submission>")]
pub fn compile(
    _token: jwt::Token,
    submission: Result<Json<CodeSubmission>, Error>,
    cache: &State<WasmCache>,
) -> Json<CompileResult> {
    // Handle any errors in the JSON submission
    let submission = match submission {
        Ok(submission) => submission.into_inner(),
        Err(e) => {
            let message = format!("Error parsing JSON: {}", e);
            return Json(CompileResult {
                success: false,
                message,
                hash: None,
                wasm: None,
            });
        }
    };

    // Create cache directory if it doesn't exist
    if !no_cache() && !cache.dir.exists() {
        fs::create_dir_all(&cache.dir).unwrap();
    }

    let lang = &submission.lang;
    let code = submission.code.trim();
    let code_hash = digest(code);
    let cache_key = format!("{}:{}", lang, code_hash);
    let cached_wasm_path = cache.dir.join(cache_key).with_extension("wasm");

    if code.len() > 100_000 {
        return Json(CompileResult {
            success: false,
            message: "Code is too long".into(),
            hash: None,
            wasm: None,
        });
    }

    // Check if cached wasm exists and return it if it does
    if !no_cache() && cached_wasm_path.exists() {
        let wasm = match fs::read(&cached_wasm_path) {
            Ok(wasm) => Some(general_purpose::STANDARD.encode(wasm)),
            Err(e) => {
                let message = format!("Error reading cached wasm: {}", e);
                return Json(CompileResult {
                    success: false,
                    message,
                    hash: None,
                    wasm: None,
                });
            }
        };

        return Json(CompileResult {
            success: true,
            message: "Compiled successfully (cached)".into(),
            hash: Some(code_hash),
            wasm,
        });
    }

    // Compile the code to wasm
    let compiler = match get_compiler_for_language(lang) {
        Ok(compiler) => compiler,
        Err(message) => {
            return Json(CompileResult {
                success: false,
                message,
                hash: None,
                wasm: None,
            });
        }
    };
    let wasm = compiler.compile(code);

    // Write the compiled wasm to the cache
    match wasm {
        Ok(wasm) => {
            if !no_cache() {
                match fs::write(&cached_wasm_path, &wasm) {
                    Ok(_) => (),
                    Err(e) => {
                        let message = format!("Error writing cached wasm: {}", e);
                        return Json(CompileResult {
                            success: false,
                            message,
                            hash: None,
                            wasm: None,
                        });
                    }
                }
            }

            let wasm_base64 = general_purpose::STANDARD.encode(&wasm);

            Json(CompileResult {
                success: true,
                message: "Compiled successfully".into(),
                hash: Some(code_hash),
                wasm: Some(wasm_base64),
            })
        }
        Err(e) => {
            let message = format!("Error compiling code: {}", e);
            Json(CompileResult {
                success: false,
                message,
                hash: None,
                wasm: None,
            })
        }
    }
}
