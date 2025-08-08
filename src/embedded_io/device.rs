use embedded_hal::i2c::I2c;
use embedded_io::{ErrorType, Read, Seek, Write};

use crate::{
    MB85RC,
    embedded_io::{error::MB85RCErrorType, head::Head},
};

pub struct EmbedIODev<T: I2c, const N: u64> {
    dev: crate::MB85RC<T>,
    head: Head<N>,
}

impl<T: I2c, const N: u64> EmbedIODev<T, N> {
    pub fn new(mb85rc: MB85RC<T>) -> Self {
        Self {
            dev: mb85rc,
            head: Head::new(),
        }
    }
}

impl<T: I2c, const N: u64> ErrorType for EmbedIODev<T, N> {
    type Error = MB85RCErrorType<T::Error>;
}

impl<T: I2c, const N: u64> Read for EmbedIODev<T, N> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        match self.head.memory_address() {
            Some(addr) => self
                .dev
                .sequential_read(&addr, buf)
                .map_err(MB85RCErrorType::I2c)
                .map(|_| {
                    self.head.advance(buf.len());
                    buf.len()
                }),
            None => Err(MB85RCErrorType::InvalidPosition),
        }
    }
}

impl<T: I2c, const N: u64> Write for EmbedIODev<T, N> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        // From trait doc: Implementations must not return Ok(0) unless buf is empty.
        if buf.is_empty() {
            return Ok(0);
        }

        // match self.head.memory_address() {
        //     Some(addr) => self
        //         .dev
        //         .write_page(&addr, buf)
        //         .map_err(MB85RCErrorType::I2c)
        //         .map(|_| {
        //             self.head.advance(buf.len());
        //             buf.len()
        //         }),
        //     None => Err(MB85RCErrorType::InvalidPosition),
        // }

        todo!()
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        // We can't really flush here.
        Ok(())
    }
}

impl<T: I2c, const N: u64> Seek for EmbedIODev<T, N> {
    fn seek(&mut self, pos: embedded_io::SeekFrom) -> Result<u64, Self::Error> {
        self.head
            .seek(pos)
            .ok_or(MB85RCErrorType::InvalidPosition)?;
        Ok(self.head.into())
    }
}
