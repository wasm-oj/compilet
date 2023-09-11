use super::core::ServerInfo;
use super::jwt::Token;
use super::openapi::SelfRoot;
use super::system::{Status, SystemInfo};
use crate::compile::{CodeSubmission, CompileResult, WasmCache};
use crate::compilers::{get_compiler_for_language, get_compilers};
use crate::config::{auto_cleanup, no_cache};
use base64::engine::general_purpose;
use base64::Engine;
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket::State;
use sha256::digest;
use std::env::temp_dir;
use std::fs;

/// Catches all OPTION requests in order to get the CORS related Fairing triggered.
#[options("/<_..>")]
pub fn all_options() -> &'static str {
    ""
}

#[get("/")]
pub fn index() -> &'static str {
    "I am Compilet. (https://github.com/wasm-oj/compilet)"
}

#[get("/openapi.yml")]
pub fn openapi_yml() -> &'static str {
    super::openapi::OPENAPI_DOCUMENT
}

/// resolve the origin and redirect to https://api-spec.pages.dev/rapidoc?url=<url>
#[get("/openapi")]
pub fn openapi(root: SelfRoot) -> Redirect {
    let url = format!(
        "https://api-spec.pages.dev/rapidoc?url={}/openapi.yml",
        root.0
    );
    Redirect::to(url)
}

#[get("/info")]
pub fn info() -> Json<ServerInfo> {
    Json(ServerInfo {
        version: env!("VERGEN_GIT_DESCRIBE").to_string(),
        commit: env!("VERGEN_GIT_SHA").to_string(),
        data: env!("VERGEN_GIT_COMMIT_TIMESTAMP").to_string(),
        os: env!("VERGEN_CARGO_TARGET_TRIPLE").to_string(),
    })
}

/// Check if the given token (in auth header) is valid
#[get("/validate")]
pub fn validate(_token: Token) -> Json<bool> {
    Json(true)
}

#[get("/system")]
pub fn system() -> Json<SystemInfo> {
    let compilers = get_compilers();

    let capabilities = compilers.iter().map(|c| (c.lang(), c.describe())).collect();

    Json(SystemInfo {
        capabilities,
        status: Status {
            compiling: 0,
            pending: 0,
        },
    })
}

// Route to handle incoming code submissions
#[post("/compile", format = "json", data = "<submission>")]
pub fn compile(
    _token: Token,
    submission: Result<Json<CodeSubmission>, rocket::serde::json::Error>,
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

    let workspace = temp_dir()
        .join("compilet")
        .join("workspace")
        .join(&code_hash);
    if !workspace.exists() {
        fs::create_dir_all(&workspace).unwrap();
    }
    #[cfg(debug_assertions)]
    dbg!(&workspace);

    let wasm = compiler.compile(code, workspace.to_str().unwrap());

    if auto_cleanup() {
        fs::remove_dir_all(&workspace).unwrap();
    }

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
