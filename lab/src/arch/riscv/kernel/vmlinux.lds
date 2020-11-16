ENTRY(_start)
BASE_ADDRESS = 0x80000000;
NR_TASKS = 5;
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
    bss_start = .;
    .bss : {
        *(.sbss .bss .bss.*)
    }
    bss_end = .;
    . += 0x8000;
    stack_top = .;
}
