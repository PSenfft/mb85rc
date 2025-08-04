#![no_std]
#![no_main]

use core::cell::RefCell;
use core::default::Default;
use embedded_hal_bus::i2c::RefCellDevice;
use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::peripherals::Peripherals;
use esp_hal::{delay::Delay, time::Duration};
use esp_println::dbg;
use esp_println::logger::init_logger;
use log::{debug, error, info};
use mb85rc::{self, MB85RC};

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    loop {
        error!("PANIC: {info}");
    }
}

esp_bootloader_esp_idf::esp_app_desc!();

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);
    init_logger(log::LevelFilter::Debug);

    let peripherals = unsafe { Peripherals::steal() };
    let i2c =
        esp_hal::i2c::master::I2c::new(peripherals.I2C0, esp_hal::i2c::master::Config::default())
            .unwrap()
            .with_sda(peripherals.GPIO22)
            .with_scl(peripherals.GPIO23);

    let delay = Delay::new();
    delay.delay(Duration::from_millis(500));
    debug!("starting...");

    let i2c_refcell = RefCell::new(i2c);

    let display_refcell_device = RefCellDevice::new(&i2c_refcell);

    debug!("wait 1s...");
    delay.delay(Duration::from_millis(1000));

    let mut mb85rc = MB85RC::new(display_refcell_device, 0x50);

    debug!("read device id");
    let device_id = mb85rc.get_device_id();
    dbg!(device_id);

    let memory_address = [0x00, 0x00];
    let data = 0xFF;

    info!("writing {} in memory address: {:?}", data, memory_address);
    let write_result = mb85rc.byte_write(memory_address, data);
    dbg!(write_result);

    delay.delay(Duration::from_secs(1));

    debug!("reading fram...");
    let read_data = mb85rc.random_read(&memory_address);
    info!("read data: {:?}", read_data);

    loop {
        delay.delay(Duration::from_secs(2));
    }
}
