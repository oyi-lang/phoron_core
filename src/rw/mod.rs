//! Module to handle the low-level reading and writing of JVM bytecode.

pub mod reader;
pub mod writer;

use std::primitive;

trait HasBeBytes<const N: usize>
where
    Self: Sized,
{
    fn from_be_bytes(buf: [u8; N]) -> Self;
    fn to_be_bytes(self) -> [u8; N];
}

impl HasBeBytes<1> for u8 {
    fn from_be_bytes(buf: [u8; 1]) -> u8 {
        primitive::u8::from_be_bytes(buf)
    }

    fn to_be_bytes(self) -> [u8; 1] {
        self.to_be_bytes()
    }
}

impl HasBeBytes<2> for u16 {
    fn from_be_bytes(buf: [u8; 2]) -> u16 {
        primitive::u16::from_be_bytes(buf)
    }

    fn to_be_bytes(self) -> [u8; 2] {
        self.to_be_bytes()
    }
}

impl HasBeBytes<4> for u32 {
    fn from_be_bytes(buf: [u8; 4]) -> u32 {
        primitive::u32::from_be_bytes(buf)
    }

    fn to_be_bytes(self) -> [u8; 4] {
        self.to_be_bytes()
    }
}

impl HasBeBytes<8> for u64 {
    fn from_be_bytes(buf: [u8; 8]) -> u64 {
        primitive::u64::from_be_bytes(buf)
    }

    fn to_be_bytes(self) -> [u8; 8] {
        self.to_be_bytes()
    }
}
