use std::{collections::VecDeque, ptr::NonNull};
use core::sync::atomic::{
    AtomicBool,
    AtomicU32,
    Ordering::{
        Acquire,
        Release,
        Relaxed
    }
};
use core::cell::UnsafeCell;

struct BaseData<T:Send>{
    data:UnsafeCell<VecDeque<T>>,
    lock:AtomicBool,
    conn:AtomicU32
}

pub struct Base<T:Send>{
    ptr:*mut BaseData<T>
}

#[repr(transparent)]
pub struct Channel;

pub struct Sender<T:Send>{
    ptr:Base<T>
}
pub struct Reciver<T:Send>{
    ptr:Base<T>
}

impl<T:Send> Clone for  Base<T>{
    fn clone(&self) -> Self {
        unsafe {
            (*self.ptr).conn.fetch_add(1, Relaxed)
        };
        Base{
            ptr:self.ptr
        }
    }
}
impl<T:Send> Drop for Base<T>{
    fn drop(&mut self) {
        unsafe {
            if (*self.ptr).conn.fetch_sub(1, Relaxed) == 1{
                drop(Box::from_raw(self.ptr))
            }
        }
    }
}

impl Channel {
    pub fn new<T:Send>()->(Sender<T>,Reciver<T>){
        todo!()
    }
}