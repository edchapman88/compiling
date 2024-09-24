#![no_std]
#![no_main]

extern crate panic_halt;

use core::arch::global_asm;

global_asm!(
    "	.syntax unified

	.arch armv7e-m 		
				

	
	.section .vectors	
	.word	__stack_top
	.word	Reset
	.word 	0
	.word	0
	.word	0
	.word	0
	.word	0
	.word	0
	.word	0
	.word	0
	.word	0	
	.word	0
	.word 	0
	.word   0	
	.word  	0
	.word	0
	
	.zero 	4 * 112		


	.text

	.global Reset
	.type Reset, %function
	
Reset:			
	.equ GPIO0, 0x50000500	
	.equ DIRSET, 0x18 
	.equ DIRCLR, 0x1c	
	.equ OUTSET, 0x08	
	.equ OUTCLR, 0x0c	
	
	.equ	PIN_21, (1<<21)
	.equ	PIN_28, (1<<28)

	ldr	r0, = 0x50000500
	movs	r1, PIN_21
	str	r1, [r0, 0x18]
	movs 	r1, PIN_28
	str	r1, [r0, 0x18 ]

	movs	r1, PIN_28
	str	r1, [r0, 0x0c	]
	movs	r1, PIN_21
	str	r1, [r0, 0x08	]	
	
forever:
	b	forever
    "
);
