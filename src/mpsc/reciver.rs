use crate::mpsc::Base;

pub struct Recieve<T:Send>{
    ptr:Base<T>
}


impl<T:Send> Recieve<T>  {
    
    pub fn new(base:Base<T>)->Self{
        Recieve { ptr: base }
    }

    pub fn read(&self)->Option<T>{
        self.ptr.recieve()
    }
}

unsafe impl<T:Send> Send for Recieve<T>{}
unsafe impl<T:Send> Sync for Recieve<T> {}