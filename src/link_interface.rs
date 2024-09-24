use core::arch::global_asm;

global_asm!(
    ".syntax unified
    
     .text
     .global Reset_Handler
     .type Reset_Handler, %function
"
);

extern "C" {
    fn Reset_Handler() -> !;
}

#[link_section = ".vectors.reset_handler"]
#[no_mangle]
pub static __RESET_HANDLER: unsafe extern "C" fn() -> ! = Reset_Handler;
