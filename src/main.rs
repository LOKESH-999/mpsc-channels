mod channel;
mod dequeue;
mod new_chanel;
mod mpsc;
use std::collections::VecDeque;
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

fn main() {
    let (sender, receiver) = mpsc::new();

    // Create an Arc to share the Sender among threads
    let sender = Arc::new(sender);

    // Barrier to synchronize the start of threads
    let barrier = Arc::new(Barrier::new(5));

    // Create 4 producer threads
    let mut handles = Vec::new();
    for i in 0..4 {
        let sender_clone = Arc::clone(&sender);
        let barrier_clone = Arc::clone(&barrier);

        let handle = thread::spawn(move || {
            // Wait for all threads to be ready
            // barrier_clone.wait();

            // Send some data
            sender_clone.send(i);
            println!("Producer {} sent data: {}", i, i);
        });

        handles.push(handle);
    }

    // Create a consumer thread
    let consumer_handle = thread::spawn(move || {
        // Wait for all producer threads to be ready
        // panic!("uio");
        // barrier.wait();
        println!("{:?}",78);
        // Allow some time for producers to send data
        thread::sleep(Duration::from_secs(1));
        // unsafe {while receiver.is_available() {}}
        while let Some(data) = receiver.read() {
            println!("Consumer received data: {}", data);
        }
    });

    // Wait for all producer threads to finish
    for handle in handles {
        handle.join().unwrap();
    }
    let mut x=VecDeque::new();
    x.push_front(1);
    x.push_front(2);
    x.push_front(3);
    while let Some(d) =  x.pop_back(){
        println!("{:?}",d)
    }
    // Wait for the consumer thread to finish
    consumer_handle.join().unwrap();
}



#[test]
fn test_sender_recieve() {
    // Create a new channel.
    let (sender, reciever) = mpsc::new();

    // Test sending and receiving data within the same thread.
    sender.send(42);
    assert_eq!(reciever.read(), Some(42));

    sender.send(100);
    assert_eq!(reciever.read(), Some(100));

    // Test receiving from an empty channel.
    assert_eq!(reciever.read(), None);

    // Test sending and receiving across threads.
    let sender_clone = sender.clone();
    let handle = thread::spawn(move || {
        sender_clone.send(99);
    });

    handle.join().unwrap();
    assert_eq!(reciever.read(), Some(99));
}
