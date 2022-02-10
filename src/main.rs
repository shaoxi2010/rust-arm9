#![no_std]
#![no_main]
#![allow(unused)]
#![feature(lang_items)]
#![feature(panic_info_message)]
mod cpu;
mod console;
use cpu::mmu::{Arm32MMU, MapType};
#[no_mangle]
#[link_section = ".mmu"]
static mmu: Arm32MMU = unsafe { Arm32MMU::new() };


#[no_mangle]
pub unsafe extern "C" fn base_init() {
    mmu.trans(0x00000000, 0x00000000, 0x80000000, cpu::mmu::MapType::NCNB.into());
    mmu.trans(0x00000000, 0x00000000, 0x10000000, cpu::mmu::MapType::CB.into());
    mmu.turn_on();
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    println!("Hello, world!");
    loop {
    }
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(args) = info.message() {
        println!("\r\nrust panic: {}", args);
    } else {
        println!("\r\nrust panic!");
    }
    loop {}
}

#[no_mangle]
pub extern "C" fn __aeabi_unwind_cpp_pr0() { }
#[lang = "eh_personality"] extern fn eh_personality() {}