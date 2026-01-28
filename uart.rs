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
        while (read_volatile(UART_FR as *const u32) & FR_BUSY) != 0 {}
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

use core::ptr::{read_volatile, write_volatile};

// RPi3 MMIO base
const MMIO_BASE: usize = 0x3F00_0000;
const GPIO_BASE: usize = MMIO_BASE + 0x0020_0000;
const UART_BASE: usize = MMIO_BASE + 0x0020_1000;

// GPIO registers
const GPFSEL1: usize = GPIO_BASE + 0x04;
const GPPUD: usize = GPIO_BASE + 0x94;
const GPPUDCLK0: usize = GPIO_BASE + 0x98;

// PL011 registers
const UART_DR: usize = UART_BASE + 0x00;
const UART_FR: usize = UART_BASE + 0x18;
const UART_IBRD: usize = UART_BASE + 0x24;
const UART_FBRD: usize = UART_BASE + 0x28;
const UART_LCRH: usize = UART_BASE + 0x2C;
const UART_CR: usize = UART_BASE + 0x30;
const UART_ICR: usize = UART_BASE + 0x44;

// FR bits
const FR_TXFF: u32 = 1 << 5;
const FR_RXFE: u32 = 1 << 4;
const FR_BUSY: u32 = 1 << 3;

#[inline(always)]
fn delay_cycles(mut n: usize) {
    while n != 0 {
        unsafe { core::arch::asm!("nop"); }
        n -= 1;
    }
}

pub unsafe fn init_uart_pl011_rpi3() {
    // 1) GPIO14/15 -> ALT0 (TX/RX)
    let mut gpfsel1 = read_volatile(GPFSEL1 as *const u32);
    // Clear FSEL14 (bits 12-14) and FSEL15 (bits 15-17)
    gpfsel1 &= !((0b111 << 12) | (0b111 << 15));
    // Set ALT0 (0b100) for both
    gpfsel1 |= (0b100 << 12) | (0b100 << 15);
    write_volatile(GPFSEL1 as *mut u32, gpfsel1);

    // 2) Disable pulls on GPIO14/15 (BCM2837 sequence)
    write_volatile(GPPUD as *mut u32, 0);
    delay_cycles(2000);
    write_volatile(GPPUDCLK0 as *mut u32, (1 << 14) | (1 << 15));
    delay_cycles(2000);
    write_volatile(GPPUD as *mut u32, 0);
    write_volatile(GPPUDCLK0 as *mut u32, 0);

    // 3) UART init
    // Wait for any ongoing TX
    while (read_volatile(UART_FR as *const u32) & FR_BUSY) != 0 {}

    // Disable UART
    write_volatile(UART_CR as *mut u32, 0);

    // Clear interrupts
    write_volatile(UART_ICR as *mut u32, 0x7FF);

    // Baud: 921_600 at 48 MHz UART clock
    write_volatile(UART_IBRD as *mut u32, 3);
    write_volatile(UART_FBRD as *mut u32, 16);

    // 8N1, FIFO enable
    write_volatile(UART_LCRH as *mut u32, (0b11 << 5) | (1 << 4));

    // Enable UART, TX, RX
    write_volatile(UART_CR as *mut u32, (1 << 0) | (1 << 8) | (1 << 9));
}

pub unsafe fn uart_write_char(c: u8) {
    while (read_volatile(UART_FR as *const u32) & FR_TXFF) != 0 {}
    write_volatile(UART_DR as *mut u32, c as u32);
}

pub unsafe fn uart_read_char_blocking() -> u8 {
    while (read_volatile(UART_FR as *const u32) & FR_RXFE) != 0 {}
    read_volatile(UART_DR as *const u32) as u8
}
