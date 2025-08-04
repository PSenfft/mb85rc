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

enum i2c_frequency {
    standard_mode = 100_000,
    fast_mode = 400_000,
    fast_mode_plus = 1_000_000,
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
fn get_device_id() -> [u8; 3] { 
 let mut id: [u8; 3] =  [1, 2, 3];

 id
}


fn byte_write(write_address: u8, data: u8) -> u8 {

    let mut read_byte: u8 = 0x00;

    read_byte

}
