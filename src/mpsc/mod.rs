#![allow(unused)]
pub mod reciver; // Module for the Receiver functionality.
pub mod sender;  // Module for the Sender functionality.

use std::{collections::VecDeque, hint::spin_loop, ptr::NonNull};
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering::*};
use core::cell::UnsafeCell;
use reciver::Recieve;
use sender::Sender;

// Core data structure managing a queue with synchronization.
struct BaseData<T: Send> {
    data: UnsafeCell<VecDeque<T>>, // Queue for storing data.
    lock: AtomicBool,              // Spinlock for synchronization.
    conn: AtomicU32                // Reference count.
}

// Wrapper for BaseData with reference counting.
#[repr(transparent)]
pub struct Base<T: Send> {
    ptr: *mut BaseData<T>, // Pointer to BaseData.
}

// Increments the reference count on clone.
impl<T: Send> Clone for Base<T> {
    fn clone(&self) -> Self {
        unsafe {
            (*self.ptr).conn.fetch_add(1, Relaxed);
        }
        Base { ptr: self.ptr }
    }
}

// Decrements the reference count and frees memory when it reaches zero.
impl<T: Send> Drop for Base<T> {
    fn drop(&mut self) {
        unsafe {
            if (*self.ptr).conn.fetch_sub(1, Relaxed) == 1 {
                drop(Box::from_raw(self.ptr));
            }
        }
    }
}

// Methods for BaseData to send and consume data safely.
impl<T: Send> BaseData<T> {
    #[inline]
    fn send(&self, data: T) {
        while self.lock.swap(true, Acquire) {
            spin_loop(); // Wait until the lock is released.
        }
        unsafe {
            (*self.data.get()).push_back(data);
        }
        self.lock.store(false, Release);
    }

    #[inline]
    fn consume(&self) -> Option<T> {
        while self.lock.swap(true, Acquire) {
            spin_loop(); // Wait until the lock is released.
        }
        let res = unsafe { (*self.data.get()).pop_front() };
        self.lock.store(false, Release);
        res
    }
}

// Cleans up resources when BaseData is dropped.
impl<T: Send> Drop for BaseData<T> {
    fn drop(&mut self) {
        self.lock.store(true, Release);
        let mut data = self.data.get_mut();
        while let Some(_) = data.pop_back() {} // Clear the queue.
    }
}

// Methods for Base to interact with BaseData.
impl<T: Send> Base<T> {
    fn new() -> Self {
        let data = BaseData {
            data: UnsafeCell::new(VecDeque::new()),
            lock: AtomicBool::new(false),
            conn: AtomicU32::new(2), // Initial reference count.
        };
        let ptr = Box::into_raw(Box::new(data));
        Base { ptr }
    }

    pub fn send(&self, data: T) {
        unsafe {
            (*self.ptr).send(data);
        }
    }

    pub fn recieve(&self) -> Option<T> {
        unsafe {
            (*self.ptr).consume()
        }
    }
}

// Creates a new Sender and Receiver pair.
pub fn new<T: Send>() -> (Sender<T>, Recieve<T>) {
    let base = Base::new();
    let sender = Sender::new(base.clone());
    let reciver = Recieve::new(base);
    (sender, reciver)
}
