use embedded_hal::i2c;
use embedded_io::Error;

#[derive(Debug)]
pub enum MB85RCErrorType<T: i2c::Error> {
    I2c(T),
    InvalidPosition,
}

impl<T: i2c::Error> Error for MB85RCErrorType<T> {
    fn kind(&self) -> embedded_io::ErrorKind {
        match self {
            MB85RCErrorType::I2c(_) => embedded_io::ErrorKind::Other,
            MB85RCErrorType::InvalidPosition => embedded_io::ErrorKind::Other,
        }
    }
}
