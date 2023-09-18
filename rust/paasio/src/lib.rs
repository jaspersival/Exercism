use std::io::{Read, Result, Write};

// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

pub struct ReadStats<R> {
    wrapped: R,
    _reads: usize,
    bytes_read: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(data: R) -> ReadStats<R> {
        ReadStats {
            wrapped: data,
            _reads: 0,
            bytes_read: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_read
    }

    pub fn reads(&self) -> usize {
        self._reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self.wrapped.read(buf) {
            Ok(bytes_read) => {
                self._reads += 1;
                self.bytes_read += bytes_read;
                Ok(bytes_read)
            }
            Err(e) => Err(e),
        }
    }
}

pub struct WriteStats<W> {
    wrapped: W,
    _writes: usize,
    bytes_written: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            wrapped,
            _writes: 0,
            bytes_written: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_written
    }

    pub fn writes(&self) -> usize {
        self._writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match self.wrapped.write(buf) {
            Ok(bytes_written) => {
                self._writes += 1;
                self.bytes_written += bytes_written;
                Ok(bytes_written)},
            Err(e) => Err(e),
        }
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
