
use std::fmt::Arguments;
use std::io::{BufWriter, IoSlice, Write};

pub struct StrWriter {
    writer: BufWriter<Vec<u8>>
}

impl StrWriter {
    pub fn new() -> StrWriter {
        StrWriter { writer: BufWriter::new(Vec::new()) }
    }

    pub fn to_string(self) -> std::io::Result<String> {
        let vec = self.writer.into_inner()?;
        String::from_utf8(vec).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }
}

impl Write for StrWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }

    fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> std::io::Result<usize> {
        self.writer.write_vectored(bufs)
    }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.writer.write_all(buf)
    }

    fn write_fmt(&mut self, fmt: Arguments<'_>) -> std::io::Result<()> {
        self.writer.write_fmt(fmt)
    }
}