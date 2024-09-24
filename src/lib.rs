#![no_std]

use core::ptr::write_volatile;

// GPIO
const GPIO_BASE_P0: u32 = 0x50000000;
const DIRSET_P0: *mut u32 = (GPIO_BASE_P0 + 0x518) as *mut u32;
const OUTSET_P0: *mut u32 = (GPIO_BASE_P0 + 0x508) as *mut u32;
const OUTCLR_P0: *mut u32 = (GPIO_BASE_P0 + 0x50C) as *mut u32;

// https://tech.microbit.org/hardware/schematic/
const _ROW1: u32 = 21;
const _COL1: u32 = 28;
const ROW3: u32 = 15;
const COL3: u32 = 31;

pub fn entry_point() {
    unsafe {
        // Set the 'direction' of the required GPIO pins to be 'output'.
        write_volatile(DIRSET_P0, 1 << ROW3);
        write_volatile(DIRSET_P0, 1 << COL3);

        // Set the column pin low and the corresponding row pin to high to light up that LED.
        write_volatile(OUTCLR_P0, 1 << COL3);
        write_volatile(OUTSET_P0, 1 << ROW3);
    }
    loop {}
}
