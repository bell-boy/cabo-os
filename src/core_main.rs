use crate::uart::write_str;

pub unsafe extern "C" fn core_main() -> ! {
    write_str("core_main called\n");
    loop {}
}