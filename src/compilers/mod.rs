pub mod c;
pub mod compiler;
pub mod cpp;
pub mod rust;

use self::c::CCompiler;
use self::compiler::Compiler;
use self::cpp::CppCompiler;
use self::rust::RustCompiler;

pub fn get_compilers() -> Vec<Box<dyn Compiler>> {
    vec![
        Box::new(RustCompiler::new()),
        Box::new(CppCompiler::new()),
        Box::new(CCompiler::new()),
    ]
}

pub fn get_compiler_for_language(language: &str) -> Result<Box<dyn Compiler>, String> {
    match language {
        "Rust" | "rs" => Ok(Box::new(RustCompiler::new())),
        "C++" | "cpp" => Ok(Box::new(CppCompiler::new())),
        "C" | "c" => Ok(Box::new(CCompiler::new())),
        _ => Err(format!("Unsupported language: {}", language)),
    }
}
