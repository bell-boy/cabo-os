pub const START: usize = 0x3F00_0000;
pub const GPIO_OFFSET: usize = 0x0020_0000;
pub const UART_OFFSET: usize = 0x0020_1000;
pub const CLOCK_SPEED: usize = 48_000_000;
pub const BAUD_RATE: usize = 115200;

pub fn write_str(string: &str) {
    for c in string.chars() {
        write_char(c);
    }
}

fn write_char(c: char) {
    unsafe {
        let ptr: *mut u8 = (START + UART_OFFSET) as *mut u8;
        core::ptr::write_volatile(ptr, c as u8);
    }
}

// claude

fn set_baud_rate(clock_freq: u32, baud: u32) {
    let divider = clock_freq / (16 * baud);
    let fraction = ((clock_freq % (16 * baud)) * 64 + (16 * baud / 2)) / (16 * baud);

    let ibrd = (START + UART_OFFSET + 0x24) as *mut u32;
    let fbrd = (START + UART_OFFSET + 0x28) as *mut u32;

    unsafe {
        core::ptr::write_volatile(ibrd, divider);
        core::ptr::write_volatile(fbrd, fraction);
    }
}

// Pseudo-code showing the concept
fn setup_uart_pins() {
    unsafe {
        // GPIO14 and GPIO15 need to be set to ALT0 function
        let gpfsel1 = (START + GPIO_OFFSET + 0x04) as *mut u32;
        let mut val = core::ptr::read_volatile(gpfsel1);

        // Clear bits for GPIO14 and GPIO15 (3 bits each)
        val &= !(0b111 << 12); // GPIO14
        val &= !(0b111 << 15); // GPIO15

        // Set to ALT0 (100 in binary)
        val |= 0b100 << 12; // GPIO14 -> TXD0
        val |= 0b100 << 15; // GPIO15 -> RXD0

        core::ptr::write_volatile(gpfsel1, val);
    }
}

pub fn init() {
    setup_uart_pins();
    set_baud_rate(CLOCK_SPEED as u32, BAUD_RATE as u32);

}
