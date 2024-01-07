#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use core::panic::PanicInfo;

use avr_device::interrupt;
use core::cell::RefCell;

type Console = arduino_hal::hal::usart::Usart0<arduino_hal::DefaultClock>;
static CONSOLE: interrupt::Mutex<RefCell<Option<Console>>> =
    interrupt::Mutex::new(RefCell::new(None));

mod gleamgrid;

macro_rules! print {
    ($($t:tt)*) => {
        interrupt::free(
            |cs| {
                if let Some(console) = CONSOLE.borrow(cs).borrow_mut().as_mut() {
                    let _ = ufmt::uwrite!(console, $($t)*);
                }
            },
        )
    };
}

macro_rules! println {
    ($($t:tt)*) => {
        interrupt::free(
            |cs| {
                if let Some(console) = CONSOLE.borrow(cs).borrow_mut().as_mut() {
                    let _ = ufmt::uwriteln!(console, $($t)*);
                }
            },
        )
    };
}

fn put_console(console: Console) {
    interrupt::free(|cs| {
        *CONSOLE.borrow(cs).borrow_mut() = Some(console);
    })
}

fn subfunction() {
    println!("We can also call println!() in a subfunction!");
}

fn demo_print_without_ln() {
    for i in 0..10 {
        print!("{} ", i);
    }
    println!("numbers!");
}

#[arduino_hal::entry]
fn main() -> ! {

    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut gleamgrid: gleamgrid::Game = gleamgrid::Game::new();

    let mut led = pins.d13.into_output();

    let serial = arduino_hal::default_serial!(dp, pins, 57600);
    put_console(serial);

    gleamgrid.for_each(&|x, y, v| {
        print!("{} ", v);
        if x == 7 {
            println!("")
        }
    });

    // println!("Hello from main!");
    // subfunction();
    // demo_print_without_ln();

    loop {

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