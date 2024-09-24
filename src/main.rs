#![no_std]
// 'binary' crates in Rust must include a function called 'main' to define the executable entry point unless the `no_main` attribute may be applied at the crate level.
#![no_main]

use compiling::entry_point;
//use core::arch::global_asm;

// #[panic_handler] is used to define the behavior of the Rust `panic!` macro (a panic is a fatal exception) in #![no_std] applications.
// https://doc.rust-lang.org/nomicon/panic-handler.html
#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[export_name = "Reset_Handler"]
pub unsafe extern "C" fn reset() {
    entry_point();
}

#[link_section = ".vectors.reset_handler"]
#[no_mangle]
pub static __RESET_HANDLER: unsafe extern "C" fn() = reset;
