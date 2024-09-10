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
