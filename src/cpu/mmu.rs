use core::mem::{transmute, zeroed};
use tock_registers::registers::{InMemoryRegister, ReadWrite};
use tock_registers::interfaces::{ReadWriteable, Writeable, Readable};

#[derive(Clone, Copy)]
pub enum MapType {
    NCNB = 0x0,
    NCB = 0x1,
    CNB = 0x2,
    CB = 0x3
}

impl Into<usize> for MapType {
    fn into(self) -> usize {
        self as usize
    }
}

#[repr(C)]
pub struct Arm32MMU {
    entry : [ReadWrite<usize>; 4096],
}

unsafe impl Sync for Arm32MMU {}

impl Arm32MMU {
    pub const unsafe fn new() -> Self{
        transmute([0; 4096]) //rust static必须是const函数
    }

    pub fn base(&self) -> usize {
        self as *const _ as usize
    }

    pub fn trans(&self, virt:usize, phys:usize, size:usize, ty: usize) {
        let mut size = size >> 20;
        let mut virt = virt >> 20;
        let mut phys = phys >> 20;

        while size != 0 {
            let v = (phys << 20) | (1 << 16) | (0x3 << 10) | (0x0 << 5) | (ty << 2) | (0x2 << 0);
            self.entry[virt].set(v);
            size -= 1;
            virt += 1;
            phys += 1;
        }
    }
    pub fn turn_on(&self) {
        super::cp15::icache_disable();
        super::cp15::dcache_disable();
        super::cp15::mmu_ttb_set(self.base());
        super::cp15::mmu_domain_set_all_manager();
        super::cache::dsb();
        super::tlb::invalid_tlb();
        super::cp15::icache_enable();
        super::cp15::dcache_enable();
        super::cp15::mmu_enable();
        super::tlb::invalid_tlb();
    }

    pub fn turn_off() {
        super::cp15::dcache_disable();
        super::cp15::icache_disable();
        super::cp15::mmu_disable();
        super::cache::invalid_icache_dcache();
        super::tlb::invalid_tlb();
        super::cache::dsb();
        super::cache::isb();
    }
}