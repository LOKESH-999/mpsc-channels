use std::ptr::NonNull;

use crate::mpsc::Base;


pub struct Sender<T:Send>{
    ptr:Base<T>
}

impl<T:Send> Clone for Sender<T>{
    fn clone(&self) -> Self {
        Self { ptr: self.ptr.clone()}
    }
}

impl<T:Send> Sender<T>{
    pub fn new(base:Base<T>)->Self{
        Self { ptr: base }
    }
    pub fn send(&self,data:T){
        self.ptr.send(data);    
    }
}

unsafe impl<T:Send> Send for Sender<T>{}
unsafe impl<T:Send> Sync for Sender<T> {}