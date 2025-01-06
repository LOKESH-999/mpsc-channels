#![allow(unused)]
pub mod reciver;
pub mod sender;


use std::{collections::VecDeque, hint::spin_loop, ptr::{fn_addr_eq, NonNull}};
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
use reciver::Recieve;
use sender::Sender;

struct BaseData<T:Send>{
    data:UnsafeCell<VecDeque<T>>,
    lock:AtomicBool,
    conn:AtomicU32
}

#[repr(transparent)]
pub struct Base<T:Send>{
    ptr:*mut BaseData<T>
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

impl<T:Send> BaseData<T> {

    #[inline]
    fn send(&self,data:T){
        while self.lock.swap(true, Acquire) {
            spin_loop();
        }
        unsafe {
            (*self.data.get()).push_back(data);   
        }
        self.lock.store(false, Release);
    }

    #[inline]
    fn consume(&self)->Option<T>{
        while self.lock.swap(true, Acquire) {
            spin_loop();
        }
        let res = unsafe {
            (*self.data.get()).pop_front()   
        };
        self.lock.store(false, Release);
        return res;
    }
}

impl<T:Send> Drop for BaseData<T> {
    fn drop(&mut self) {
        self.lock.store(true, Release);
        let mut data = self.data.get_mut();
        while let Some(data) = data.pop_back() {}

    }
}


impl<T:Send> Base<T>{
    fn new()->Self{
        let data = BaseData{
            data:UnsafeCell::new(VecDeque::new()),
            lock:AtomicBool::new(false),
            conn:AtomicU32::new(2)
        };
        let ptr = Box::into_raw(Box::new(data));
        Base { ptr }
    }

    pub fn send(&self,data:T){
        unsafe {
            (*self.ptr).send(data);
        }
    }

    pub fn recieve(&self)->Option<T>{
        unsafe {
            (*self.ptr).consume()
        }
    }
}

pub fn new<T:Send>()->(Sender<T>,Recieve<T>){
    let base = Base::new();
    let sender = Sender::new(base.clone());
    let reciver = Recieve::new(base);
    (sender,reciver)
}