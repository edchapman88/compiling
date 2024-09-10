// IMPORTANT the standard `main` interface is not used because it requires nightly.
#![no_main]
// Don't use the standard library when runnning without an OS.
#![no_std]

// Unwinding panics are not supported without the standard library
//#[panic_handler]
//fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
//    loop {}
//}

extern crate panic_halt;

use core::ptr::{read_volatile, write_volatile};
// use microbit::{board::Board, hal::prelude::OutputPin};

// GPIO
const GPIO_BASE_P0: u32 = 0x50000000;
const GPIO_BASE_P1: u32 = 0x50000300;
const DIRSET_P0: *mut u32 = (GPIO_BASE_P0 + 0x518) as *mut u32;
const DIRSET_P1: *mut u32 = (GPIO_BASE_P1 + 0x518) as *mut u32;
const OUTSET_P0: *mut u32 = (GPIO_BASE_P0 + 0x508) as *mut u32;
const OUTCLR_P0: *mut u32 = (GPIO_BASE_P0 + 0x50C) as *mut u32;

// UART
const UART_BASE: u32 = 0x40002000;
const UART_ENABLE: *mut u32 = (UART_BASE + 0x500) as *mut u32;
const UART_PINSELTXD: *mut u32 = (UART_BASE + 0x50C) as *mut u32;
const UART_STARTTX: *mut u32 = (UART_BASE + 0x008) as *mut u32;
const UART_TXD: *mut u32 = (UART_BASE + 0x51C) as *mut u32;
const UART_TXREADY: *mut u32 = (UART_BASE + 0x11C) as *mut u32;
const BAUD: *mut u32 = (UART_BASE + 0x524) as *mut u32;

// https://tech.microbit.org/hardware/schematic/
const ROW1: u32 = 21;
const COL1: u32 = 28;
const _ROW3: u32 = 15;
const _COL3: u32 = 31;

fn uart_send(x: u32) {
    unsafe {
        while read_volatile(UART_TXREADY) == 0 {}
        write_volatile(UART_TXREADY, 0);
        write_volatile(UART_TXD, x);
    }
}

// The `#[entry]` attribute macro is provided by the `cortex_m_rt` crate as a program entry point *after* static variable have been initialised.
#[cortex_m_rt::entry]
// The entry point is not allowed to return.
fn entry_point() -> ! {
    // let mut board = Board::take().unwrap();
    // board.display_pins.col3.set_low().unwrap();
    // board.display_pins.row3.set_high().unwrap();
    unsafe {
        write_volatile(DIRSET_P0, 1 << ROW1);
        write_volatile(DIRSET_P0, 1 << COL1);

        write_volatile(OUTCLR_P0, 1 << COL1);
        write_volatile(OUTSET_P0, 1 << ROW1);

        // UART
        write_volatile(DIRSET_P1, 1 << 8);
        write_volatile(BAUD, 0x01D7E000);
        write_volatile(UART_PINSELTXD, 0x28);
        write_volatile(UART_ENABLE, 4);
        write_volatile(UART_STARTTX, 1);
        write_volatile(UART_TXD, 0x00);
    }
    loop {
        uart_send(0x30 + (1 & 7));
    }
}
