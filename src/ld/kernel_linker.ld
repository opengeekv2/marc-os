OUTPUT_ARCH( "riscv" )
ENTRY(start)
MEMORY
{
  ram(wxa) : ORIGIN = 0x80000000, LENGTH = 4K
}
SECTIONS
{
  . = 0x80000000;
  .text : { 
    *(.text.init)
    *(.text)
  }
  .data : {
    . = ALIGN(4096);
    *(.data)
  }
  .bss : {
    PROVIDE(_bss_start = .);
    *(.bss)
    PROVIDE(_bss_end = .);
  }
  PROVIDE(_memory_start = ORIGIN(ram));
  PROVIDE(_stack_start = _bss_end);
  PROVIDE(_stack_end = _stack_start + 0x80000);
  PROVIDE(_memory_end = ORIGIN(ram) + LENGTH(ram));
  PROVIDE(_heap_start = _stack_end);
  PROVIDE(_heap_size = _memory_end - _heap_start);
}
