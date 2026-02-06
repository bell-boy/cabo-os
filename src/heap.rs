#![allow(dead_code)]

use core::ptr::null_mut;

/// Simple single-threaded bump allocator.
///
/// Safety: This allocator does not enforce aliasing rules or synchronization.
/// Callers must ensure exclusive access and proper alignment usage.
pub struct BumpAllocator {
    start: usize,
    end: usize,
    next: usize,
}

impl BumpAllocator {
    /// Create an empty allocator that must be initialized before use.
    pub const fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            next: 0,
        }
    }

    /// Initialize the allocator with a memory region.
    ///
    /// # Safety
    /// Caller must provide a valid, writable region of `size` bytes starting at `start`.
    pub unsafe fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start.saturating_add(size);
        self.next = start;
    }

    /// Allocate `size` bytes with `align` alignment. Returns null on failure.
    ///
    /// # Safety
    /// Caller must ensure exclusive access to the allocator and that `align` is a power of two.
    pub unsafe fn alloc(&mut self, size: usize, align: usize) -> *mut u8 {
        if align == 0 || !align.is_power_of_two() {
            return null_mut();
        }

        let aligned = align_up(self.next, align);
        let new_next = match aligned.checked_add(size) {
            Some(v) => v,
            None => return null_mut(),
        };

        if new_next > self.end {
            return null_mut();
        }

        self.next = new_next;
        aligned as *mut u8
    }

    /// Reset the allocator back to the start of the region.
    ///
    /// # Safety
    /// Caller must ensure no live allocations remain.
    pub unsafe fn reset(&mut self) {
        self.next = self.start;
    }

    /// Bytes remaining in the region.
    pub fn remaining(&self) -> usize {
        self.end.saturating_sub(self.next)
    }

    /// Start address of the region.
    pub fn start(&self) -> usize {
        self.start
    }

    /// End address (exclusive) of the region.
    pub fn end(&self) -> usize {
        self.end
    }

    /// Current bump pointer.
    pub fn current(&self) -> usize {
        self.next
    }
}

#[inline]
fn align_up(addr: usize, align: usize) -> usize {
    // align is assumed to be a power of two
    (addr + (align - 1)) & !(align - 1)
}
