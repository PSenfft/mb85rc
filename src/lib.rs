use embedded_hal::i2c::I2c;

const DEVICE_ADDRESS: u8 = 0b10100000;
const DEVICE_ADDRESS_CODE: u8 = 0b00000000;
const DEVICE_W: u8 = 0b00000000;
const DEVICE_R: u8 = 0b00000001;

struct MB85RC<T: I2c> {
    i2c: T,
}

impl<T: I2c> MB85RC<T> {
    pub fn new(i2c: T) -> Self {
        MB85RC { i2c }
    }
}
