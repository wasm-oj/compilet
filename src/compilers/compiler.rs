pub trait Compiler {
    fn compile(&self, source: &str) -> Result<Vec<u8>, String>;
}
