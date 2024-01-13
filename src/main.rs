#![no_std]
#![no_main]
extern crate arduino_hal;

use core::panic::PanicInfo;
use arduino_hal::prelude::_embedded_hal_blocking_i2c_Write;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{

    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );

    let baseadr = 0x20;
    let _res0 = i2c.write(baseadr, &[0x00,0x00,0x00]).unwrap();
    arduino_hal::delay_ms(1); // obviously, this time is needed b4 the outsputs are usable
    let _res5 = i2c.write(baseadr, &[0x12,0x0,0x0]);
    arduino_hal::delay_ms(1000);
    loop {
        let _res2 = i2c.write(baseadr, &[0x12,0x00,0x00]);
        arduino_hal::delay_ms(1000);
        let _res4 = i2c.write(baseadr, &[0x12,0xff,0xff]);
        arduino_hal::delay_ms(500);
    }
}