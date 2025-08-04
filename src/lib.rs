use std::convert::Infallible;

use embedded_hal::i2c::{I2c, SevenBitAddress};
use embedded_io::{ErrorType, Read};

const DEVICE_ADDRESS: u8 = 0b10100000;
const DEVICE_ADDRESS_CODE: u8 = 0b00000000;
const DEVICE_W: u8 = 0b00000000;
const DEVICE_R: u8 = 0b00000001;

struct MB85RC<T: I2c<SevenBitAddress>> {
    i2c: T,
}

impl<T: I2c> MB85RC<T> {
    pub fn new(i2c: T) -> Self {
        MB85RC { i2c }
    }
}

impl<T: I2c> ErrorType for MB85RC<T> {
    type Error = Infallible;
}

impl<T: I2c> Read for MB85RC<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        todo!()
    }
}
