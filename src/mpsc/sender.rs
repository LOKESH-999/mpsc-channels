use std::ptr::NonNull;
use crate::mpsc::Base;

// Structure representing the sending end of the channel.
pub struct Sender<T: Send> {
    ptr: Base<T>, // Shared reference to the Base for sending data.
}

impl<T: Send> Clone for Sender<T> {
    // Creates a new Sender by cloning the shared Base reference.
    fn clone(&self) -> Self {
        Self { ptr: self.ptr.clone() }
    }
}

impl<T: Send> Sender<T> {
    // Creates a new Sender instance from a given Base.
    pub fn new(base: Base<T>) -> Self {
        Self { ptr: base }
    }

    // Sends data into the channel.
    pub fn send(&self, data: T) {
        self.ptr.send(data);
    }
}

// Safe to transfer Sender across threads.
unsafe impl<T: Send> Send for Sender<T> {}

// Safe to share Sender across threads since its operations are synchronized.
unsafe impl<T: Send> Sync for Sender<T> {}
