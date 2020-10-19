ENTRY(_start)
BASE_ADDRESS = 0x80000000;
SECTIONS
{   
    . = BASE_ADDRESS;
    .text : {
        *(.text.init)
        *(.text.entry)
        *(.text .text.*)
    }
    .rodata : {
        *(.rodata .rodata.*)
    }
    .data : {
        *(.data .data.*)
    }
    .bss : {
        *(.sbss .bss .bss.*)
    }
    . += 0x8000;
    stack_top = .;
}
