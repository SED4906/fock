#![allow(internal_features)]
#![feature(
    core_intrinsics,
    naked_functions,
    abi_x86_interrupt,
    pointer_is_aligned
)]
#![no_std]
#![no_main]

#[cfg_attr(target_arch = "x86_64", path = "arch/x86_64/cpu.rs")]
mod cpu;
mod helper;
mod irq;
mod mm;
#[cfg_attr(target_arch = "x86_64", path = "arch/x86_64/serial.rs")]
mod serial;
mod bf;

extern crate alloc;

use core::panic::PanicInfo;

use alloc::{string::{String, ToString}, vec::Vec};
use nom::multi::many1;

static MODULE_REQUEST: limine::request::ModuleRequest = limine::request::ModuleRequest::new();

#[no_mangle]
extern "C" fn _start() -> ! {
    unsafe {
        serial::serial_init();
    }
    println!("ok");
    mm::arch::mm_init();
    println!("mm");
    irq::arch::irq_init();
    println!("irq");
    cpu::cpu_init();
    println!("cpu");
    // Language runtime below
    for module in MODULE_REQUEST.get_response().unwrap().modules() {
        unsafe {
            let string = String::from_utf8_lossy(core::slice::from_raw_parts(module.addr(),module.size() as usize)).to_string();
            let src = many1(bf::parse)(string.as_str());
            let mut program = bf::Interpreter {
                mem: Vec::new(),
                p: 0,
            };
            program.run(src.unwrap().1);
        }
    }
    // Language runtime above
    println!("done!");
    loop {
        unsafe {
            #[cfg(target_arch = "x86_64")]
            x86::halt();
        }
    }
}

#[panic_handler]
unsafe fn rust_panic(info: &PanicInfo) -> ! {
    println!("{info}");
    hcf()
}

unsafe fn hcf() -> ! {
    #[cfg(target_arch = "x86_64")]
    x86::irq::disable();
    loop {
        #[cfg(target_arch = "x86_64")]
        x86::halt();
    }
}