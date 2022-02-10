use core::arch::asm;

#[inline(always)]
pub fn invalid_tlb() {
    let empty:usize = 0;
    unsafe {
        asm!(
        "mcr p15, 0, {0}, c8, c7, 0",
        in(reg) empty,
        options(preserves_flags, nostack)
        );
    }
}

#[inline(always)]
pub fn invalid_itlb() {
    let empty:usize = 0;
    unsafe {
        asm!(
        "mcr p15, 0, {0}, c8, c5, 0",
        in(reg) empty,
        options(preserves_flags, nostack)
        );
    }
}

#[inline(always)]
pub fn invalid_dtlb() {
    let empty:usize = 0;
    unsafe {
        asm!(
        "mcr p15, 0, {0}, c8, c6, 0",
        in(reg) empty,
        options(preserves_flags, nostack)
        );
    }
}