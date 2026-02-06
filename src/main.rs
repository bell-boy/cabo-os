#![no_main]
#![no_std]

use core::arch::{global_asm, asm};
mod synch;
mod uart;
use core::panic::PanicInfo;
use uart::write_str;
use core::fmt::Write;

use crate::uart::QemuWriter;
mod core_main;
mod heap;

global_asm!(include_str!("boot.S"));

unsafe extern "C" {
    unsafe static wait_loop: fn() -> !;
}

static mut heap: [u8; 6 * 1024 * 1024] = [0; 6 * 1024 * 1024];

#[repr(C)]
struct CpuInfo {
    ready: usize, // what the waiting cores will read
    goto_address: unsafe extern "C" fn() -> !, // what they'll jump to
    sp: usize, // stack pointer
    core_id: usize
}

// cores search through this based on their core_id
#[unsafe(no_mangle)]
static mut core_mailbox: [CpuInfo; 4] = [
    CpuInfo { ready: 0, goto_address: core_main::core_main, sp: 0, core_id: 0 },
    CpuInfo { ready: 0, goto_address: core_main::core_main, sp: 0, core_id: 1 },
    CpuInfo { ready: 0, goto_address: core_main::core_main, sp: 0, core_id: 2 },
    CpuInfo { ready: 0, goto_address: core_main::core_main, sp: 0, core_id: 3 },
];

/*  init function, never returns, and uses C abi, which defines things like caller registers, etc.
so it can be called from assembly in boot.S */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_main() -> () {

    let mut writer = QemuWriter {};
    // let _ = write!(writer, "Hello, world!");
    // let _ = write!(writer, "core_main addr: {:x}", core_main::core_main as usize);

    let heap_start = core::ptr::addr_of_mut!(heap) as usize;
    let heap_size = core::mem::size_of::<[u8; 6 * 1024 * 1024]>();
    let mut allocator = heap::BumpAllocator::new();
    unsafe {
        allocator.init(heap_start, heap_size);
    }

    // initialize stacks for cores 1-3 and set their ready flag to 1 so they can jump to core_main
    for i in 1..4 {
        unsafe {
            let stack_top = allocator.alloc(0x80000, 16) as usize + 0x80000;
            core_mailbox[i].sp = stack_top as usize;
            core_mailbox[i].goto_address = core_main::core_main;
            asm!("dsb ishst", options(nostack, preserves_flags));
            let entry = core::ptr::addr_of!(wait_loop) as usize;
            core::ptr::write_volatile((0xd8 + (i * 0x8)) as *mut usize, entry);
            core_mailbox[i].ready = 1;
            asm!("dsb ishst", options(nostack, preserves_flags));

            asm!("sev", options(nostack, preserves_flags));
        }
    }
    loop {}

}

//--------------------------------------------------------------------------------------------------
// Private Code
//--------------------------------------------------------------------------------------------------

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    write_str("Kernel panic: ");
    loop {}
}
