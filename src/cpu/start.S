	.global _start
	.section .text._start
_start:
    /* Enter svc mode and mask interrupts */
    mrs r0, cpsr
    bic r0, r0, #0x1f
    orr r0, r0, #0xd3
    msr cpsr, r0
    /* Clear BSS Section for rust */
    ldr	r0, =__bss_start
    ldr	r1, =__bss_end		/* this is link info */
    mov	r2, #0x00000000		/* prepare zero to clear BSS */
clbss_l:
    cmp	r0, r1			/* while not at end of BSS */
    strlo	r2, [r0]		/* clear 32-bit BSS word */
    addlo	r0, r0, #4		/* move to next */
    blo	clbss_l
    ldr r0, =0x80000  /*stack top!!!!!*/
    bic	r0, r0, #7	/* 8-byte alignment for ABI compliance */
    mov	sp, r0
    bl base_init
    bl main
