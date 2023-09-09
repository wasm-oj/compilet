/// This trait represents a compiler varient.
pub trait Compiler {
    /// Returns the programming language supported by the compiler.
    /// (The file extension without the dot, in lowercase. e.g. `rs`)
    fn lang(&self) -> String;

    /// Returns a description of the compiler.
    fn describe(&self) -> String;

    /// Compiles the given source code string.
    ///
    /// # Arguments
    ///
    /// * `source` - A reference to the source code string to be compiled.
    /// * `workspace` - A reference to the working directory to use for compilation.
    ///
    /// # Returns
    ///
    /// Returns a `Result` type. The `Ok` variant contains a vector of bytes representing the compiled output,
    /// while the `Err` variant contains an error message as a string.
    fn compile(&self, source: &str, workspace: &str) -> Result<Vec<u8>, String>;
}
