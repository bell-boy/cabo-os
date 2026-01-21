
pub const START: usize = 0x3F00_0000;
pub const GPIO_OFFSET: usize = 0x0020_0000;
pub const UART_OFFSET: usize = 0x0020_1000;

pub fn write_str(string: &str) {
    for c in string.chars() {
        write_char(c);
    }
}

fn write_char(c: char) {
    unsafe {
        let ptr: *mut u8 = UART_OFFSET as *mut u8;
        core::ptr::write_volatile(ptr, c as u8);
    }
}
