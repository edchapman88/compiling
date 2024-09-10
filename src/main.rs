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

use core::ptr::write_volatile;
// use microbit::{board::Board, hal::prelude::OutputPin};

const GPIO_BASE_VAL: u32 = 0x50000000;
const _GPIO_BASE: *mut u32 = GPIO_BASE_VAL as *mut u32;
const DIRSET_OFFSET: *mut u32 = (GPIO_BASE_VAL + 0x518) as *mut u32;
const OUTSET_OFFSET: *mut u32 = (GPIO_BASE_VAL + 0x508) as *mut u32;
const OUTCLR_OFFSET: *mut u32 = (GPIO_BASE_VAL + 0x50C) as *mut u32;

// https://tech.microbit.org/hardware/schematic/
const ROW1: u32 = 21;
const COL1: u32 = 28;
const _ROW3: u32 = 15;
const _COL3: u32 = 31;

// The `#[entry]` attribute macro is provided by the `cortex_m_rt` crate as a program entry point *after* static variable have been initialised.
#[cortex_m_rt::entry]
// The entry point is not allowed to return.
fn entry_point() -> ! {
    // let mut board = Board::take().unwrap();
    // board.display_pins.col3.set_low().unwrap();
    // board.display_pins.row3.set_high().unwrap();
    unsafe {
        write_volatile(DIRSET_OFFSET, 1 << ROW1);
        write_volatile(DIRSET_OFFSET, 1 << COL1);

        write_volatile(OUTCLR_OFFSET, 1 << COL1);
        write_volatile(OUTSET_OFFSET, 1 << ROW1);
    }
    loop {}
}
