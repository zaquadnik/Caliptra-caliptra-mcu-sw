/*++

Licensed under the Apache-2.0 license.

File Name:

    main.rs

Abstract:

    File contains entry point for bare-metal RISCV program

--*/

#![no_std]
#![no_main]

use core::arch::global_asm;
use core::ptr;

global_asm!(include_str!("start.S"));

const OUT_STR: &[u8; 14] = b"Hello Caliptra";

#[no_mangle]
pub extern "C" fn main() {
    const UART0: *mut u8 = 0x2000_1041 as *mut u8;
    unsafe {
        for byte in OUT_STR {
            ptr::write_volatile(UART0, *byte);
        }
        ptr::write_volatile(UART0, b'\n');
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
