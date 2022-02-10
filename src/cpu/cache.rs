use core::arch::asm;

#[inline(always)]
pub fn invalid_icache_dcache() {
    let empty:usize = 0;
    unsafe {
        asm!(
        "mcr p15, 0, {0}, c7, c7, 0",
        in(reg) empty,
        options(preserves_flags, nostack)
        );
    }
}
#[inline(always)]
pub fn invalid_icache() {
    let empty:usize = 0;
    unsafe {
        asm!(
        "mcr p15, 0, {0}, c7, c5, 0",
        in(reg) empty,
        options(preserves_flags, nostack)
        );
    }
}
#[inline(always)]
pub fn invalid_dcache() {
    let empty:usize = 0;
    unsafe {
        asm!(
        "mcr p15, 0, {0}, c7, c6, 0",
        in(reg) empty,
        options(preserves_flags, nostack)
        );
    }
}
#[inline(always)]
pub fn dsb() {
    let empty:usize = 0;
    unsafe {
        asm!(
        "mcr p15, 0, {0}, c7, c10, 4",
        in(reg) empty,
        options(preserves_flags, nostack)
        );
    }
}
#[inline(always)]
pub fn isb() {
    /* arm9 does not have this */
}

#[inline(always)]
fn _get_cache() -> usize {
    let cache:usize;
    unsafe {
        asm!(
        "mrc p15, 0, {0}, c0, c0, 1", out(reg) cache,
        options(preserves_flags, nostack)
        );
    }
    cache
}
#[inline(always)]
fn _flush_range(start: usize, stop: usize, line: usize) {
    let start = start & !(line - 1);
    let stop = if stop & (line - 1) != 0 {(stop + line) & !(line -1)} else { stop };
    for mva in (start..stop).step_by(line) {
        unsafe { asm!("mcr p15, 0, {0}, c7, c14, 1", in(reg)mva);}
    }
}
#[inline(always)]
fn _invalid_range(start: usize, stop: usize, line: usize) {
    let start = start & !(line - 1);
    let stop = if stop & (line - 1) != 0 {(stop + line) & !(line -1)} else { stop };
    for mva in (start..stop).step_by(line) {
        unsafe { asm!("mcr p15, 0, {0}, c7, c6, 1", in(reg)mva) }
    }
}
#[inline(always)]
pub fn flush_range(start: usize, stop: usize) {
    let cache = _get_cache();
    let line = 1 << ((cache & 0x3) + 3);
    _flush_range(start, stop, line);
    dsb();
}
#[inline(always)]
pub fn invalid_range(start: usize, stop: usize) {
    let cache = _get_cache();
    let line = 1 << ((cache & 0x3) + 3);
    _invalid_range(start, stop, line);
    dsb();
}