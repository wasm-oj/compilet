use compilers::get_compiler_for_language;
use sha256::digest;
use std::{env::temp_dir, fs, path::PathBuf};

mod cli;
mod compile;
mod compilers;
mod config;
mod server;
mod version;

#[macro_use]
extern crate rocket;

#[rocket::main]
async fn main() {
    let matches = cli::cli().get_matches();

    match matches.subcommand() {
        Some(("run", _)) => {
            let _ = server::core::rocket().launch().await;
        }
        Some(("compile", m)) => {
            let source: &PathBuf = m.get_one("source").unwrap();
            let output: &PathBuf = m.get_one("output").unwrap();

            let source = source.to_owned();
            let output = output.to_owned();
            eprintln!(
                "Compiling {} into {} ...",
                source.display(),
                output.display()
            );

            let ext = source.extension().unwrap().to_str().unwrap();
            let compiler =
                get_compiler_for_language(ext).expect("No compiler found for this language.");

            let code = fs::read_to_string(source).expect("Unable to read source file.");
            let code_hash = digest(code.clone());

            let workspace = temp_dir()
                .join("compilet")
                .join("workspace")
                .join(code_hash);
            if !workspace.exists() {
                fs::create_dir_all(&workspace).unwrap();
            }

            let wasm = compiler
                .compile(code.as_str(), workspace.to_str().unwrap())
                .expect("Unable to compile source file.");

            fs::remove_dir_all(&workspace).unwrap();

            fs::write(output, wasm).expect("Unable to write output file.");
            eprintln!("Done!");
        }
        _ => {
            let _ = server::core::rocket().launch().await;
        }
    }
}
