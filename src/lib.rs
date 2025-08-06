#![no_std]

use core::result::Result;
use embedded_hal::i2c::{I2c, SevenBitAddress};

const DEVICE_ADDRESS: u8 = 0b10100000;
const DEVICE_ADDRESS_CODE: u8 = 0b00000000;
const DEVICE_W: u8 = 0b00000000;
const DEVICE_R: u8 = 0b00000001;

pub struct MB85RC<T: I2c<SevenBitAddress>> {
    i2c: T,
    address: SevenBitAddress,
}

impl<T: I2c> MB85RC<T> {
    pub fn new(i2c: T, address: SevenBitAddress) -> Self {
        MB85RC { i2c, address }
    }

    /// The Device ID command reads fixed Device ID. The size of Device ID is 3 bytes and consists of manufacturer
    /// ID and product ID.
    ///  # Arguments
    /// * `self` - A mutable reference to the MB85RC instance.
    /// # Returns
    /// * `Result<[u8; 3], Error>` - Device ID is 3 bytes and consists of manufacturer ID and product ID
    pub fn get_device_id(&mut self) -> Result<[u8; 3], T::Error> {
        let mut buffer: [u8; 3] = [0, 0, 0];
        let reserved_slave_address = 0x7C; // Reserved Slave ID F9H without last bit, because wrte address adds this bit 
        let payload = [0xA0]; // Device Address + read bit (write bit works also, because R/W code are “Don't care” value)
        self.i2c
            .write_read(reserved_slave_address, &payload, &mut buffer)?;

        Ok(buffer)
    }

    /// Write bit on the specified memory address
    /// # Arguments
    /// * `self` - A mutable reference to the MB85RC instance.
    /// * `memory_address` - The memory address to write to.
    /// * `data` - The data byte to write.
    /// # Returns
    /// * `Result<u8, T::Error>`
    pub fn byte_write(&mut self, memory_address: [u8; 2], data: u8) -> Result<u8, T::Error> {
        let payload = [memory_address[0], memory_address[1], data];
        self.i2c.write(self.address, &payload)?;

        Ok(data)
    }

    /// If additional 8 bits are continuously sent after the same command (except stop condition) as Byte Write, a
    /// page write is performed. The memory address rolls over to first memory address (0000H) at the end of the
    /// address. Therefore, if more than 32 Kbytes are sent, the data is overwritten in order starting from the start
    /// of the memory address that was written first. Because FRAM performs the high-speed write operations, the
    /// data will be written to FRAM right after the ACK response finished.
    /// array 32KB
    /// # Arguments
    /// * `self` - A mutable reference to the MB85RC instance.
    /// * `memory_address` - The memory address to write to.
    /// * `data` - The data bytes to write max 32KB.
    /// # Returns
    /// * `Result<(), T::Error>`
    pub fn write_page(
        &mut self,
        memory_address: [u8; 2],
        data: &mut [u8; 32000],
    ) -> Result<(), T::Error> {
        let mut payload = [0u8; 32002];
        payload[0] = memory_address[0];
        payload[1] = memory_address[1];
        payload[2..].copy_from_slice(data);
        self.i2c.write(self.address, &payload)?;

        Ok(())
    }

    /// The one byte of data from the memory address saved in the memory address buffer can be read out
    /// synchronously
    /// # Arguments
    /// * `self` - A mutable reference to the MB85RC instance.
    /// * `memory_address` - The memory address to read from.
    /// # Returns
    /// * `Result<u8, Error>` - The byte read from the specified
    pub fn random_read(&mut self, memory_address: &[u8; 2]) -> Result<u8, T::Error> {
        let mut buffer: [u8; 1] = [0];
        self.i2c
            .write_read(self.address, memory_address, &mut buffer)?;

        Ok(buffer[0])
    }

    /// Performs a sequential read operation starting from the specified memory address,
    /// reading data continuously into the provided buffer. After specifying the address,
    /// data can be received continuously following the device address word with a read
    /// command. If the end of the memory address space is reached, the internal read
    /// address automatically rolls over to the first memory address (0x0000) and continues
    /// reading.
    /// # Arguments
    /// * `self` - A mutable reference to the MB85RC instance.
    /// * `memory_address` - The memory address to read from.
    /// * `buffer` - buffer to write the payload data
    /// # Return
    /// * `Result<&'a mut [u8], T::Error>` - Pointer to the array with the read data
    pub fn sequential_read<'a>(
        &mut self,
        memory_address: &[u8; 2],
        buffer: &'a mut [u8],
    ) -> Result<&'a mut [u8], T::Error> {
        self.i2c.write_read(self.address, memory_address, buffer)?;

        Ok(buffer)
    }
}
