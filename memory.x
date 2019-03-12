MEMORY
{
  /* 
   * Leave 8k for the default bootloader.
   * https://learn.adafruit.com/adafruit-trinket-m0-circuitpython-arduino/uf2-bootloader-details#making-your-own-uf2-38-42
   */
  FLASH (rx) : ORIGIN = 0x00000000 + 8K, LENGTH = 256K - 8K
  RAM (xrw)  : ORIGIN = 0x20000000, LENGTH = 32K
}
_stack_start = ORIGIN(RAM) + LENGTH(RAM);
