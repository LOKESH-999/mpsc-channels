use crate::mpsc::Base;

// Structure representing the receiving end of the channel.
pub struct Recieve<T: Send> {
    ptr: Base<T>, // Shared reference to the Base for accessing data.
}

impl<T: Send> Recieve<T> {
    // Creates a new Recieve instance from a given Base.
    pub fn new(base: Base<T>) -> Self {
        Recieve { ptr: base }
    }

    // Reads and removes a value from the queue, if available.
    pub fn read(&self) -> Option<T> {
        self.ptr.recieve()
    }
}

// Safe to transfer Recieve across threads.
unsafe impl<T: Send> Send for Recieve<T> {}

// Safe to share Recieve across threads since its operations are synchronized.
unsafe impl<T: Send> Sync for Recieve<T> {}
