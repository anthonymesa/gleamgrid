#![no_std]
#![no_main]

use heapless::String;
use core::panic::PanicInfo;

mod gleamgrid;

#[arduino_hal::entry]
fn main() -> ! {

    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut gleamgrid: gleamgrid::Game = gleamgrid::Game::new();

    let mut led = pins.d13.into_output();
    
    loop {

        let board_string: String<56> = gleamgrid.board_as_string();

        for ch in board_string.chars() {
            if let Some(digit) = ch.to_digit(10) {
                if digit <= 5 {
                    let number = digit as u8;
                    for _ in 0..(number * 2) {
                        led.toggle();
                        arduino_hal::delay_ms(100);
                    }          
                }
            }

            arduino_hal::delay_ms(500);
        }

        arduino_hal::delay_ms(2000);

        gleamgrid.update_board();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut led = pins.d13.into_output();

    loop {
        led.set_high();
        arduino_hal::delay_ms(50);
        led.set_low();
        arduino_hal::delay_ms(50);
    }
}