#![no_main]
#![no_std]


/*  init function, never returns, and uses C abi, which defines things like caller registers, etc.
so it can be called from assembly in boot.S */
#[no_mangle]
pub unsafe extern "C" fn kmain() -> ! {
    panic!();
}

// dummy panic handler 
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    cpu::wait_forever()
}