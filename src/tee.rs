use std::io::{Read, Result, Write};

/// TeeReader takes in a [std::io::Read](https://doc.rust-lang.org/std/io/trait.Read.html) and a [std::io::Write](https://doc.rust-lang.org/std/io/trait.Write.html) and mirrors all "reads" to the writer
///
/// This is useful if you want to log all reads from a `Read`er to some file (the `Write`r)
pub struct TeeReader<R, W> {
    read: R,
    output: W,
    force_flush: bool,
}

impl<R, W> TeeReader<R, W> {
    /// Create a new `TeeReader` from the `Read`er and `Write`r
    ///
    /// If `force_flush` is enabled then all writes get flushed
    pub fn new(read: R, output: W, force_flush: bool) -> Self {
        Self {
            read,
            output,
            force_flush,
        }
    }

    /// Moves the wrapped Read and Write out
    pub fn into_inner(self) -> (R, W) {
        (self.read, self.output)
    }
}

impl<R: Read, W: Write> Read for TeeReader<R, W> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n = self.read.read(buf)?;
        self.output.write_all(&buf[..n])?;
        if self.force_flush {
            self.output.flush()?;
        }
        Ok(n)
    }
}

impl<R: Read + Clone, W: Write + Clone> Clone for TeeReader<R, W> {
    fn clone(&self) -> Self {
        Self {
            read: self.read.clone(),
            output: self.output.clone(),
            force_flush: self.force_flush,
        }
    }
}

/// TeeWriter takes two [std::io::Write](https://doc.rust-lang.org/std/io/trait.Write.html) and mirrors all "writes" to the other writer
///
/// This is useful if you want to log all writes from a `Write`er to some file (the `Write`r)
///
pub struct TeeWriter<L, R> {
    write: L,
    output: R,
}

impl<L, R> TeeWriter<L, R> {
    /// Create a new TeeWriter from two Write impls.
    pub fn new(write: L, output: R) -> Self {
        Self { write, output }
    }
}

impl<L: Write, R: Write> Write for TeeWriter<L, R> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let n = self.write.write(buf)?;
        let _ = self.output.write(&buf[..n])?;
        Ok(n)
    }

    fn flush(&mut self) -> Result<()> {
        self.write.flush()
    }
}
