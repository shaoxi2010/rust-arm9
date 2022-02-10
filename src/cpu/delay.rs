use core::arch::asm;

/* make cpu delay some cycles */
#[inline(always)]
pub fn delay(cyc: usize) {
    let real_cyc = 1 + cyc / 2;
    unsafe {
        asm!(
        "1:",
        "subs {}, #1",
        "bne 1b",
        inout(reg) real_cyc => _
        );
    }
}