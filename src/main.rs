#![no_main]
#![no_std]
#![feature(lang_items)]

use core::arch::asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[no_mangle]
pub extern "C" fn KernelMain() -> ! {
    loop {
        unsafe { asm!("hlt") }
    }
}
