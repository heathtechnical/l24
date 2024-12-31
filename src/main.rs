#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod early_print;
mod limine;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    assert!(limine::BASE_REVISION.is_supported());

    early_print!("Hello World!\n");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
