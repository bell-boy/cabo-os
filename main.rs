#![no_main]
#![no_std]

use core::arch::global_asm;
mod synch;
mod uart;
use uart::write_str;

global_asm!(include_str!("boot.S"));


/*  init function, never returns, and uses C abi, which defines things like caller registers, etc.
so it can be called from assembly in boot.S */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmain() -> () {
    uart::init();
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
