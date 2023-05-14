use std::path::PathBuf;

use clap::{arg, value_parser, Command};

use shadow_rs::shadow;

shadow!(build);

pub fn cli() -> Command {
    let command = Command::new("compilet")
        .version(build::CLAP_LONG_VERSION)
        .about("Server that compiles Rust, C, and C++ into WebAssembly.")
        .author("Jacob Lin <jacob@csie.cool>")
        .subcommand(Command::new("run").about("Run the Compilet server. (default)"))
        .subcommand(
            Command::new("compile")
            .alias("c")
            .about("Compile a source file into WebAssembly.")
            .args(&[
                arg!(-o --output <output> "Output file. If no \".wasm\" is provided, it will be added automatically.").default_value(
                    "app.wasm"
                ).value_parser(
                    |output: &str| -> Result<String, String> {
                        if output.ends_with(".wasm") {
                            Ok(output.to_string())
                        } else {
                            Ok(format!("{}.wasm", output))
                        }
                    }
                ).value_parser(value_parser!(PathBuf)),
                arg!(<source> "Source file to compile.").value_parser(value_parser!(PathBuf))
            ])
        );

    return command;
}
