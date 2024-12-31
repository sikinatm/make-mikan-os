OUTPUT_FORMAT(elf64-x86-64)
ENTRY(kernel_main)

SECTIONS {

  . = 0x100000 + SIZEOF_HEADERS;

  .text : {
    *(.text .text.*)
    *(.data .data.*)
    *(.rodata .rodata.*)
    *(.bss .bss.*)
  }
}