use embedded_hal::i2c::I2c;
use embedded_io::{ErrorType, Read, Seek, Write};

use crate::{
    MB85RC,
    embedded_io::{error::MB85RCErrorType, head::Head},
};

pub struct EmbedIODev<T: I2c> {
    dev: crate::MB85RC<T>,
    head: Head,
}

impl<T: I2c> EmbedIODev<T> {
    pub fn new(mb85rc: MB85RC<T>) -> Self {
        Self {
            dev: mb85rc,
            head: Head::new(),
        }
    }
}

impl<T: I2c> ErrorType for EmbedIODev<T> {
    type Error = MB85RCErrorType<T::Error>;
}

impl<T: I2c> Read for EmbedIODev<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.dev
            .sequential_read(self.head.memory_address().unwrap(), buf)
            .map_err(MB85RCErrorType::I2c)
            .inspect(|read_bytes| {
                self.head.advance(*read_bytes);
            })
    }
}

impl<T: I2c> Write for EmbedIODev<T> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        // From trait doc: Implementations must not return Ok(0) unless buf is empty. 
        if buf.is_empty() {
            return Ok(0);
        }

        todo!()
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        todo!()
    }
}

impl<T: I2c> Seek for EmbedIODev<T> {
    fn seek(&mut self, pos: embedded_io::SeekFrom) -> Result<u64, Self::Error> {
        self.head
            .seek(pos)
            .map_err(|_| MB85RCErrorType::InvalidPosition)?;
        Ok(self.head.into())
    }
}
