use std::collections::VecDeque;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool,AtomicUsize,Ordering::{Acquire,Release}};
use std::alloc::{Layout,alloc,dealloc};
// use std::mem::forget;

pub struct Base<T>{
    data:VecDeque<T>,
    lock:AtomicBool,
    conn:AtomicUsize
}

#[repr(transparent)]
pub struct Channel;

#[repr(transparent)]
pub struct Sender<T>{
    ptr:NonNull<Base<T>>
}
#[repr(transparent)]
pub struct Reciver<T>{
    ptr:NonNull<Base<T>>
}

impl Channel{
    pub fn new<T>()->(Sender<T>,Reciver<T>){
        

        let b=Base{
            data:VecDeque::<T>::with_capacity(15),
            lock:AtomicBool::new(false),
            conn:AtomicUsize::new(2),
        };
        let ptr=unsafe {
            let lay=Layout::new::<Base<T>>();
            alloc(lay) as *mut Base<T>
        };
        
        (Sender{ptr:NonNull::new(ptr).unwrap()},Reciver{ptr:NonNull::new(ptr).unwrap()})
    }
}

impl<T> Drop for Sender<T>{
    fn drop(&mut self) {
        unsafe {
            let base=self.ptr.read();
            while base.lock.swap(true, Acquire) {}
            base.conn.fetch_sub(1, Release);
            if base.conn.load(Acquire)==0{
                let lay=Layout::new::<Base<T>>();
                dealloc(self.ptr.as_ptr() as *mut u8, lay)
            }else{
                base.lock.store(false, Release)
            }

        }
    }
}
impl<T> Sender<T>{
    pub fn clone(&self)->Self{
        unsafe {
            let base=self.ptr.read();
            base.conn.fetch_add(1, Release);
            Sender{
                ptr:self.ptr
            }
        }
    }
}

impl<T> Drop for Reciver<T>{
    fn drop(&mut self) {
        unsafe {
            let base=self.ptr.read();
            while base.lock.swap(true, Acquire) {}
            base.conn.fetch_sub(1, Release);
            if base.conn.load(Acquire)==0{
                let lay=Layout::new::<Base<T>>();
                dealloc(self.ptr.as_ptr() as *mut u8, lay)
            }else{
                base.lock.store(false, Release)
            }

        }
    }
}