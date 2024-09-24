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
// mod lib;
use core::arch::global_asm;
use core::ptr::{read_volatile, write_volatile};
// pub(crate) mod interface;
// pub(crate) mod link_interface;
// use microbit::{board::Board, hal::prelude::OutputPin};

global_asm!(
    ".syntax unified
    
     .section .vectors
     .word __stack_top
     .word Reset

     .text
     .global Reset
     .type Reset, %function

    Reset:
        bl main"
);

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

// Can send 8 bits at once
fn uart_send(x: u8) {
    unsafe {
        while read_volatile(UART_TXREADY) == 0 {}
        write_volatile(UART_TXREADY, 0);
        write_volatile(UART_TXD, x as u32);
    }
}

#[export_name = "main"]
pub unsafe extern "C" fn entry_point() {
    //#[cortex_m_rt::entry]
    //fn entry() -> ! {
    unsafe {
        write_volatile(DIRSET_P0, 1 << ROW1);
        write_volatile(DIRSET_P0, 1 << COL1);

        write_volatile(OUTCLR_P0, 1 << COL1);
        write_volatile(OUTSET_P0, 1 << ROW1);

        // UART
        write_volatile(DIRSET_P1, 1 << 8);
        write_volatile(BAUD, 0x01D7E000);
        write_volatile(UART_PINSELTXD, 0x00000000 | (1 << 5) | 8);
        write_volatile(UART_ENABLE, 4);
        write_volatile(UART_STARTTX, 1);
        write_volatile(UART_TXD, 0x00);
    }
    loop {
        // Any `char` in Rust can be represented by 4 bytes.
        // let mut buffer = [0_u8; 4];
        // let c = 'h'.encode_utf8(&buffer);
        uart_send(0x30 + (1 & 7));
    }
}
