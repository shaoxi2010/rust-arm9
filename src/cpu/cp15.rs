use tock_registers::register_bitfields;
use tock_registers::registers::{InMemoryRegister};
use tock_registers::interfaces::{ReadWriteable, Writeable, Readable};
use core::arch::asm;

register_bitfields!{
    u32,
    CP1 [
        MMU     0,
        Align   1,
        DCache  2,
        BE      7,
        System_Protection   8,
        Rom_Protection      9,
        ICache  12,
        HVector 13,
    ],
}

#[inline(always)]
fn write_p15_c1(reg :InMemoryRegister<u32, CP1::Register>) {
    unsafe {
        asm!("mcr p15, 0, {0}, c1, c0, 0", in(reg) reg.get(),
        options(preserves_flags, nostack));
    }
}
#[inline(always)]
fn read_p15_c1() -> InMemoryRegister<u32, CP1::Register>{
    let mut value;
    unsafe {
        asm!("mrc p15, 0, {0}, c1, c0, 0", out(reg) value,
        options(preserves_flags, nostack));
    }
    InMemoryRegister::new(value)
}

#[inline(always)]
pub fn mmu_enable() {
    let reg = read_p15_c1();
    reg.modify(CP1::MMU::SET);
    write_p15_c1(reg);
}
#[inline(always)]
pub fn mmu_disable() {
    let reg = read_p15_c1();
    reg.modify(CP1::MMU::CLEAR);
    write_p15_c1(reg);
}
#[inline(always)]
pub fn icache_enable() {
    let reg = read_p15_c1();
    reg.modify(CP1::ICache::SET);
    write_p15_c1(reg);
}
#[inline(always)]
pub fn icache_disable() {
    let reg = read_p15_c1();
    reg.modify(CP1::ICache::CLEAR);
    write_p15_c1(reg);
}
#[inline(always)]
pub fn dcache_enable() {
    let reg = read_p15_c1();
    reg.modify(CP1::DCache::SET);
    write_p15_c1(reg);
}
#[inline(always)]
pub fn dcache_disable() {
    let reg = read_p15_c1();
    reg.modify(CP1::DCache::CLEAR);
    write_p15_c1(reg);
}

#[inline(always)]
pub(crate) fn mmu_ttb_set(base: usize) {
    unsafe {
        asm!("mcr p15, 0, {0}, c2, c0, 0", in(reg) base);
    }
}

register_bitfields!{
    u32,
    CP3 [
        D0 OFFSET(0) NUMBITS(2) [NoAccess = 0, Client = 1, Manager = 3],
        D1 OFFSET(2) NUMBITS(2) [NoAccess = 0, Client = 1, Manager = 3],
        D2 OFFSET(4) NUMBITS(2) [NoAccess = 0, Client = 1, Manager = 3],
        D3 OFFSET(6) NUMBITS(2) [NoAccess = 0, Client = 1, Manager = 3],
        D4 OFFSET(8) NUMBITS(2) [NoAccess = 0, Client = 1, Manager = 3],
        D5 OFFSET(10) NUMBITS(2) [NoAccess = 0, Client = 1, Manager = 3],
        D6 OFFSET(12) NUMBITS(2) [NoAccess = 0, Client = 1, Manager = 3],
        D7 OFFSET(14) NUMBITS(2) [NoAccess = 0, Client = 1, Manager = 3],
        D8 OFFSET(16) NUMBITS(2) [NoAccess = 0, Client = 1, Manager = 3],
        D9 OFFSET(18) NUMBITS(2) [NoAccess = 0, Client = 1, Manager = 3],
        D10 OFFSET(20) NUMBITS(2) [NoAccess = 0, Client = 1, Manager = 3],
        D11 OFFSET(22) NUMBITS(2) [NoAccess = 0, Client = 1, Manager = 3],
        D12 OFFSET(24) NUMBITS(2) [NoAccess = 0, Client = 1, Manager = 3],
        D13 OFFSET(26) NUMBITS(2) [NoAccess = 0, Client = 1, Manager = 3],
        D14 OFFSET(28) NUMBITS(2) [NoAccess = 0, Client = 1, Manager = 3],
        D15 OFFSET(30) NUMBITS(2) [NoAccess = 0, Client = 1, Manager = 3],
    ],
}
#[inline(always)]
fn write_p15_c3(reg :InMemoryRegister<u32, CP3::Register>) {
    unsafe {
        asm!("mcr p15, 0, {0}, c3, c0, 0", in(reg) reg.get(),
        options(preserves_flags, nostack));
    }
}
#[inline(always)]
fn read_p15_c3() -> InMemoryRegister<u32, CP3::Register>{
    let mut value;
    unsafe {
        asm!("mrc p15, 0, {0}, c3, c0, 0", out(reg) value,
        options(preserves_flags, nostack));
    }
    InMemoryRegister::new(value)
}

#[inline(always)]
pub(crate) fn mmu_domain_set_all_manager() {
    let reg = read_p15_c3();
    reg.write(CP3::D0::Manager + CP3::D1::Manager
        + CP3::D2::Manager + CP3::D3::Manager
        + CP3::D4::Manager + CP3::D5::Manager
        + CP3::D6::Manager + CP3::D7::Manager
        + CP3::D8::Manager + CP3::D9::Manager
        + CP3::D10::Manager + CP3::D11::Manager
        + CP3::D12::Manager + CP3::D13::Manager
        + CP3::D14::Manager + CP3::D15::Manager
    );
    write_p15_c3(reg);
}
