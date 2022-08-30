mod decoder;
pub mod error;
mod varint;

pub trait Reader: std::io::Read + std::io::Seek {}

pub fn compile_module<R: Reader>(bytes: &mut R) -> Result<(), error::Error> {
    let _module = decoder::Module::new(bytes)?;
    todo!("Still need to add compiler and export generator");
}
