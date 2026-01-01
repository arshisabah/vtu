/* STM32F103C8T6 Memory Configuration */
MEMORY
{
  /* 64KB Flash (some use 128KB but official spec is 64KB) */
  FLASH : ORIGIN = 0x08000000, LENGTH = 64K
  
  /* 20KB SRAM */
  RAM : ORIGIN = 0x20000000, LENGTH = 20K
}

/* Stack size - cortex-m-rt will place at end of RAM */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);