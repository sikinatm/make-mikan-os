#![no_std]
#![no_main]

use core::arch::asm;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}