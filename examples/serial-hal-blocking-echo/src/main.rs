#![no_main]
#![no_std]

use panic_halt as _;

use core::fmt::Write;
use cortex_m_rt::entry;
use embedded_io::Read;

#[cfg(feature = "v1")]
use microbit::{
    hal::uart,
    hal::uart::{Baudrate, Parity},
};

#[cfg(feature = "v2")]
use microbit::{
    hal::uarte,
    hal::uarte::{Baudrate, Parity},
};

#[cfg(feature = "v2")]
mod serial_setup;
#[cfg(feature = "v2")]
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    let board = microbit::Board::take().unwrap();

    #[cfg(feature = "v1")]
    let mut serial = {
        uart::Uart::new(
            board.UART0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        )
    };

    #[cfg(feature = "v2")]
    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    loop {
        write!(serial, "Hello World:\r\n").unwrap();
        let mut input = [0];
        serial.read_exact(&mut input).unwrap();
        write!(serial, "You said: {}\r\n", input[0] as char).unwrap();
    }
}
