pub trait Compiler {
    fn lang(&self) -> &'static str;
    fn describe(&self) -> &'static str;
    fn compile(&self, source: &str) -> Result<Vec<u8>, String>;
}
