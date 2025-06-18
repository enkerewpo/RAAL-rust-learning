use std::cell::UnsafeCell;
use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct SpinLock<T> {
    locked: AtomicBool,
    v: UnsafeCell<T>, // interior mutability
}

// our guard cannot outlive the lock
pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> Deref for Guard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.v.get() }
    }
}
impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.v.get() }
    }
}

impl<'a, T> Drop for Guard<'a, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}
// sync: multiple threads can access the same data
// send: the data can be sent to another thread, only one thread can access the data at a time, so we only require T: Send
// because we are spinlocks, and spinlocks only let one thread access the data at a time

impl<T> SpinLock<T> {
    pub fn new(v: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            v: UnsafeCell::new(v),
        }
    }
    pub fn lock<'a>(&'a self) -> Guard<'a, T> {
        while self.locked.swap(true, Ordering::Acquire) {
            std::hint::spin_loop(); // tell compiler to not optimize this loop!
            // but this hint is depended on hardware
        }
        Guard { lock: self }
    }
}
