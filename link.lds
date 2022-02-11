OUTPUT_FORMAT("elf32-littlearm", "elf32-bigarm", "elf32-littlearm")
OUTPUT_ARCH(arm)
ENTRY(_start)

SECTIONS {
    . = 0x00000000;
    .text :{
        *(.text._start)
        *(.text*)
    }
    . = ALIGN(4);
    .data : {
        *(.data*)
    }
    . = ALIGN(0x4000);
    .mmu : {
        *(.mmu*)
    }
    . = ALIGN(4);
    .rodata : {
        *(.rodata*)
    }
    . = ALIGN(4);
    __bss_start = .;
    .bss : {
        *(.bss*)
    }
    __bss_end = .;
    . = . + 1M;
    __stack_top = ALIGN(1M);
    ASSERT(. <= (10M), "image has exceeded its limit.")
}