use core::arch::global_asm;

global_asm!(
    ".syntax unified
    
     .text
     .global Reset
     .type Reset, %function
"
);

extern "C" {
    fn Reset() -> !;
}

#[link_section = ".vectors.reset_vector"]
#[no_mangle]
pub static __RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;
