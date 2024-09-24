## Bits, Bytes, Hex and Memory Addressing
A bit is a `0` or `1`.

A byte is 8 bits, 00000000.

Decimal is a base-10 number system, with digits 0,1,2,3,4,5,6,7,8,9, forming numbers like `34`. A two digit decimal number existing in the range 0 -> 99, which is `100` (10^2) possible numbers.

Hex, short for Hexadecimal, is a base-16 number system, with digits 0,1,2,3,4,5,6,7,8,9,A,B,C,D,E,F, forming numbers like `0x34`, `0xC4`. They can also be written with a trailing h like `5Ch`. Since each digit represents 16 numbers, there are 256 (16^2) numbers in the range `0x0` -> `0xFF`. Some useful hex numbers to recognise are:

| Hex | Binary   |
| --- | -------- |
| 3   | 11       |
| 7   | 111      |
| 8   | 1000     | 
| F   | 1111     |
| 10  | 10000    |
| 1F  | 11111    |
| FF  | 11111111 |

'Word' size is a property of a computer architecture and denotes the number of bits that a CPU can _process at one time_. The word size also usually corresponds to the address size of the computer. '32-bit' computers have a word size of 32 and usually use 32-bit memory addresses. **Normally these computers are 'byte-addressable', meaning each of the 2^32 memory addresses point to one byte (8 bits, 2 hex digits) of memory.**

## LED
The LED display is acces via the GPIO pins.
- P0 GPIO_BASE = 0x50000000
- P1 GPIO_BASE = 0x50000300
- P0 covers P0.00 to P0.31
- P0.15 = row3
- P0.31 = col3

## UART
"A UART transmission sequence is started by triggering the STARTTX task.

Bytes are transmitted by writing to the TXD register. When a byte has been successfully transmitted, the UART will generate a TXDRDY event after which a new byte can be written to the TXD register. A UART transmission sequence is stopped immediately by triggering the STOPTX task."
from https://docs.nordicsemi.com/bundle/ps_nrf52833/page/uart.html

- P0.06 = UART_INT_RX
- P1.08 = UART_INT_TX
