pub mod compiler;

#[cfg(feature = "c")]
pub mod c;

#[cfg(feature = "cpp")]
pub mod cpp;

#[cfg(feature = "rs")]
pub mod rs;

use self::compiler::Compiler;

#[cfg(feature = "c")]
use self::c::CCompiler;

#[cfg(feature = "cpp")]
use self::cpp::CppCompiler;

#[cfg(feature = "rs")]
use self::rs::RsCompiler;

pub fn get_compilers() -> Vec<Box<dyn Compiler>> {
    vec![
        #[cfg(feature = "rs")]
        Box::new(RsCompiler::new()),
        #[cfg(feature = "cpp")]
        Box::new(CppCompiler::new()),
        #[cfg(feature = "c")]
        Box::new(CCompiler::new()),
    ]
}

pub fn get_compiler_for_language(language: &str) -> Result<Box<dyn Compiler>, String> {
    // use Compiler's lang() method to determine which compiler to use
    for compiler in get_compilers() {
        if compiler.lang() == language {
            return Ok(compiler);
        }
    }

    // if we get here, we don't have a compiler for the requested language
    Err(format!("Unsupported language: {}", language))
}
