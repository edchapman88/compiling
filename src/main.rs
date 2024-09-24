#![no_main]
#![no_std]

use core::arch::global_asm;

use compiling::entry_point;

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

global_asm!(
    ".syntax unified
    
     .text
     .global Reset_Handler
     .type Reset_Handler, %function
"
);

#[export_name = "Reset_Handler"]
pub unsafe extern "C" fn reset() {
    entry_point()
}

#[link_section = ".vectors.reset_handler"]
#[no_mangle]
pub static __RESET_HANDLER: unsafe extern "C" fn() = reset;
