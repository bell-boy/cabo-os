use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool, Ordering::Acquire, Ordering::Release};
use core::ops::{Deref, DerefMut};

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
            value: UnsafeCell::new(value)
        }
    }

    // in order to acquire a reference to T, acquire the lock
    pub fn lock(&self) -> Guard<'_, T> {
        // test and set loop
        while (self.locked.swap(true, Acquire)) {
            core::hint::spin_loop();
        }
        return Guard {lock: self};
    }
}

// a wrapper around the reference to T for the user to simply drop
// instead of calling unlock, meaning there is no dangled &mut reference
pub struct Guard<'a, T> {
    lock: &'a SpinLock<T> // 'a means SpinLock must be alive as long as Guard is
}

// implement deref trait for guard so that when you dereference a guard, you just get the type
impl<T> Deref for Guard<'_, T> {
    // this trait has an associated type that returns for deref
    type Target = T;
    fn deref(&self) -> &T{
        unsafe {
            return &*self.lock.value.get();
        }
    }
}

// both of these are safe because this object is only created when the thread has "acquired" the lock
impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T{
        unsafe {
            return &mut *self.lock.value.get();
        }
    }
}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Release)
    }
}

// only implement send and sync if T has it, since this guard is essentially just T
unsafe impl<T> Send for Guard<'_, T> where T: Send {}
unsafe impl<T> Sync for Guard<'_, T> where T: Sync {}