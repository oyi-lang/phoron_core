//! A module for writing Java (JVM) types to a byte stream.

use super::HasBeBytes;
use crate::error::WriteError;
use std::io::Write;

pub type WriteResult<T> = Result<T, WriteError>;

/// The `Writer` is used to write the bytes of a class file to a stream
pub struct Writer<'a, W: Write> {
    writer: &'a mut W,
}

impl<'a, W: Write> Writer<'a, W> {
    pub fn new(writer: &'a mut W) -> Self {
        Writer { writer }
    }

    fn write_n<T: HasBeBytes<N>, const N: usize>(&mut self, data: T) -> WriteResult<()> {
        let buf = data.to_be_bytes();
        let len = std::mem::size_of::<T>();

        if buf.len() != len {
            return Err(WriteError::new(format!(
                "error while writing bytes to stream: expected {} bytes, but found {}",
                len,
                buf.len()
            )));
        }

        self.writer.write(&buf)?;
        Ok(())
    }

    /// Write an unsigned byte (8 bits) to the byte stream.
    pub fn write_unsigned_byte(&mut self, b: u8) -> WriteResult<()> {
        self.write_n::<u8, 1>(b)
    }

    /// Write an unsigned short (16 bits) to the byte stream.
    pub fn write_unsigned_short(&mut self, s: u16) -> WriteResult<()> {
        self.write_n::<u16, 2>(s)
    }

    /// Write an unsigned int (32 bits) to the byte stream.
    pub fn write_unsigned_int(&mut self, i: u32) -> WriteResult<()> {
        self.write_n::<u32, 4>(i)
    }

    /// Write an unsigned long (64 bits) to the byte stream.
    pub fn write_unsigned_long(&mut self, l: u64) -> WriteResult<()> {
        self.write_n::<u64, 8>(l)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_unsigned_byte() {
        let mut buf = Vec::new();

        let mut writer = Writer::new(&mut buf);
        let _ = writer.write_unsigned_byte(0xca);
        let _ = writer.write_unsigned_byte(0xfe);
        let _ = writer.write_unsigned_byte(0xba);
        let _ = writer.write_unsigned_byte(0xbe);

        assert_eq!(buf, &[0xca, 0xfe, 0xba, 0xbe]);
    }

    #[test]
    fn test_write_unsigned_short() {
        let mut buf = Vec::new();

        let mut writer = Writer::new(&mut buf);
        let _ = writer.write_unsigned_short(0xcafe);
        let _ = writer.write_unsigned_short(0xbabe);

        assert_eq!(buf, &[0xca, 0xfe, 0xba, 0xbe]);
    }

    #[test]
    fn test_write_unsigned_int() {
        let mut buf = Vec::new();

        let mut writer = Writer::new(&mut buf);
        let _ = writer.write_unsigned_int(0xcafebabe);

        assert_eq!(buf, &[0xca, 0xfe, 0xba, 0xbe]);
    }

    #[test]
    fn test_write_unsigned_long() {
        let mut buf = Vec::new();

        let mut writer = Writer::new(&mut buf);
        let _ = writer.write_unsigned_long(0x00000041000f0a00);

        assert_eq!(buf, &[0x00, 0x00, 0x00, 0x41, 0x00, 0x0f, 0x0a, 0x00]);
    }
}
