/* 
BBC micro:bit v2 linker script
Written by the Hut23 Compiler Club
Do not edit this file directly; instead edit the source ".org" file 
*/

MEMORY {
  FLASH    (rx)  : ORIGIN = 0x00000000, LENGTH = 512K  
  RAM      (rwx) : ORIGIN = 0x20000000, LENGTH = 128K
  CODE_RAM (rwx) : ORIGIN = 0x00800000, LENGTH = 128K 
}


SECTIONS {
  .text : {
    LONG(__stack_top);
    KEEP(*(.vectors.reset_handler));
    *(.text*)
    *(.rodata*)
  } >FLASH
  .data : ALIGN(4) {
      *(.data)
      *(.data.*)
      . = ALIGN(4);
  } >RAM AT >FLASH
  .bss : ALIGN(4) {
    *(.bss)
    *(.bss.*);
    . = ALIGN(4);
  } >RAM

  /DISCARD/ :
  {
    *(.debug_*);
    *(.ARM.*);
    *(.comment);
  }

}

/* All the memory from the end of bss to the top of RAM */
__heap_start = .;
__stack_top = ORIGIN(RAM) + LENGTH(RAM);

/* VMA of the .data section */
__data_start = ADDR(.data); 
__data_end   = __data_start + SIZEOF(.data);

/* LMA of the .data section */
__data_load_start = LOADADDR(.data);

/* VMA of the .bss section */
__bss_start = ADDR(.bss);
__bss_end   = __bss_start + SIZEOF(.bss);

/* Entry point (for gdb) */
ENTRY(Reset_Handler);
