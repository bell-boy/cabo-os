#![no_main]
#![no_std]

use core::arch::global_asm;

global_asm!(include_str!("boot.S"));

pub const START: usize = 0x3F00_0000;
pub const GPIO_OFFSET: usize = 0x0020_0000;
pub const UART_OFFSET: usize = 0x0020_1000;

fn write_str(string: &str) {
    for c in string.chars() {
        write_char(c);
    }
}

fn write_char(c: char) {
    unsafe {
        let ptr: *mut u8 = 0x3F201000 as *mut u8;
        core::ptr::write_volatile(ptr, c as u8);
    }
}

/*  init function, never returns, and uses C abi, which defines things like caller registers, etc.
so it can be called from assembly in boot.S */
#[no_mangle]
pub unsafe extern "C" fn kmain() -> () {
    let string: &str = "Hello World!";
    write_str(string);
}

use core::panic::PanicInfo;

//--------------------------------------------------------------------------------------------------
// Private Code
//--------------------------------------------------------------------------------------------------

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unimplemented!()
}
