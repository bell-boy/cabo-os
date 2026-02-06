#![no_main]
#![no_std]

use core::arch::global_asm;
mod synch;
mod uart;
use core::panic::PanicInfo;
use uart::write_str;
mod core_main;
mod heap;

global_asm!(include_str!("boot.S"));

static heap: [u8; 6 * 1024 * 1024] = [0; 6 * 1024 * 1024];

#[repr(C)]
struct CpuInfo {
    ready: usize, // what the waiting cores will read
    goto_address: unsafe extern "C" fn() -> !, // what they'll jump to
    sp: usize, // stack pointer
    core_id: usize
}

// cores search through this based on their core_id
static core_mailbox: [CpuInfo; 4] = [
    CpuInfo { ready: 0, goto_address: core_main::core_main, sp: 0, core_id: 0 },
    CpuInfo { ready: 0, goto_address: core_main::core_main, sp: 0, core_id: 1 },
    CpuInfo { ready: 0, goto_address: core_main::core_main, sp: 0, core_id: 2 },
    CpuInfo { ready: 0, goto_address: core_main::core_main, sp: 0, core_id: 3 },
];

/*  init function, never returns, and uses C abi, which defines things like caller registers, etc.
so it can be called from assembly in boot.S */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_main() -> () {

    write_str("Hello World!\n");

    let heap_start: usize = heap.as_ptr() as usize;
    let heap_size: usize = heap.len();
    let mut allocator = heap::BumpAllocator::new();
    unsafe {
        allocator.init(heap_start, heap_size);
    }

    // initialize stacks for cores 1-3 and set their ready flag to 1 so they can jump to core_main
    for i in 1..4 {
        unsafe {
            let stack_top = allocator.alloc(0x80000, 16) + 0x80000;
            core_mailbox[i].sp = stack_top as usize;
            core_mailbox[i].ready = 1;
        }
    }
    loop {}

}

//--------------------------------------------------------------------------------------------------
// Private Code
//--------------------------------------------------------------------------------------------------

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
