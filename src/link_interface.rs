use core::arch::global_asm;

global_asm!(
    ".syntax unified
    
     .section .vectors
     .word __stack_top
     .word Reset_Handler
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
     .global Reset_Handler
     .type Reset_Handler, %function
"
);
