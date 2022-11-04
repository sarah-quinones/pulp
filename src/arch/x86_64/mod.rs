pub use super::x86::*;

pub trait Sse41X64: Sse41 {}

impl<T: Sse41> Sse41X64 for T {}
