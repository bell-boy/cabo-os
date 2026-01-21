use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering}

// locks will be used to gain mutex access to a resource T
pub struct SpinLock<T> {
    locked: AtomicBool,

    // type that can give out mutable references to different owners, bypassing rust's ownerships
    // requirements
    value: UnsafeCell<T>
}

unsafe impl<T>  Sync for SpinLock<T> where T: Send {}

impl<T> SpinLock<T> {
    fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: value
        }
    }

    pub fn lock(&self) -> &mut T {
        // test and set loop
        while (self.locked.swap(true, Acquire)) {;}
        unsafe {
            // get the value T and wrap it in a mutable reference to return to user
            // must be unsafe because compiler can't check there aren't other references 
            return &mut *self.value.get();
        }
    }
    
}

