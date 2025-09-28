ENTRY(_start)

SECTIONS
{
    . = 0x100000;  /* Load at 1MB */
    
    .text : {
        *(.text .text.*)
    }
    
    .rodata : {
        *(.rodata .rodata.*)
    }
    
    .data : {
        *(.data .data.*)
    }
    
    .bss : {
        *(.bss .bss.*)
        *(COMMON)
    }
    
    /DISCARD/ : {
        *(.note .note.*)
        *(.comment .comment.*)
    }
}
