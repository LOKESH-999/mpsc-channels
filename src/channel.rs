use std::collections::VecDeque;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool,AtomicUsize,Ordering::{Acquire,Release}};
use std::alloc::{Layout,alloc,dealloc};
use core::hint::spin_loop;
use core::cell::UnsafeCell;
// use std::mem::forget;

pub struct Base<T>{
    data:UnsafeCell<VecDeque<T>>,
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

impl<T> Sender<T>{
    pub fn send(&self,data:T){
        unsafe {
            let mut x=self.ptr.as_ref();
            while x.lock.swap(true, Acquire){
                spin_loop()
            }
            &mut(*x.data.get()).push_front(data);
            x.lock.store(false,Release)
        }
    }
}
impl<T> Reciver<T>{
    pub fn read(&self)->Option<T>{
        unsafe {
            let mut x=self.ptr.read();
            while x.lock.swap(true, Acquire) {
                spin_loop()
            }
            let d=(*x.data.get()).pop_back();
            x.lock.store(false, Release);
            d
        }
    }
    pub unsafe fn is_available(&self)->bool{
        unsafe {
            !((*self.ptr.as_ref().data.get()).len()>0)
        }
    }
}
impl Channel{
    pub fn new<T>()->(Sender<T>,Reciver<T>){
        let b=Base{
            data:UnsafeCell::new(VecDeque::<T>::with_capacity(15)),
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
            // let base=self.ptr.read();
            while self.ptr.read_volatile().lock.swap(true, Acquire) {
                spin_loop()
            }
            self.ptr.read_volatile().conn.fetch_sub(1, Release);
            if self.ptr.read_volatile().conn.load(Acquire)==0{
                let lay=Layout::new::<Base<T>>();
                dealloc(self.ptr.as_ptr() as *mut u8, lay)
            }else{
                self.ptr.read().lock.store(false, Release)
            }

        }
    }
}
impl<T> Sender<T>{
    pub fn clone(&self)->Self{
        unsafe {
            // let base=self.ptr.read();
            //todo overflow
            self.ptr.read().conn.fetch_add(1, Release);
            Sender{
                ptr:NonNull::new(self.ptr.as_ptr()).unwrap()
            }
        }
    }
}

impl<T> Drop for Reciver<T>{
    fn drop(&mut self) {
        unsafe {
            let base=self.ptr.read();
            while self.ptr.read_volatile().lock.swap(true, Acquire) {
                spin_loop()
            }
            self.ptr.read().conn.fetch_sub(1, Release);
            if self.ptr.read().conn.load(Acquire)==0{
                let lay=Layout::new::<Base<T>>();
                dealloc(self.ptr.as_ptr() as *mut u8, lay)
            }else{
                self.ptr.read().lock.store(false, Release)
            }
        }
    }
}

unsafe impl<T> Send for Sender<T> {}
unsafe impl<T> Send for Reciver<T> {}
unsafe impl<T> Sync for Reciver<T>{}
unsafe impl<T> Sync for Sender<T>{}