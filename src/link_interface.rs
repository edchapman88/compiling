use core::arch::global_asm;

global_asm!(
    ".syntax unified
    
     .section .vectors
     .word __stack_top
     .word Reset
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
     .zero 4 * 112

     .text
     .global Reset
     .type Reset, %function

    Reset:
        bl main"
);
