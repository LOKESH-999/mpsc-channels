# MPSC (Multi-Producer, Single-Consumer) Channel in Rust

This project implements a thread-safe multi-producer, single-consumer (MPSC) channel in Rust. It provides a lightweight, low-level abstraction for inter-thread communication without relying on external libraries or crates.

## Features
- **Sender-Receiver Model**: The channel consists of `Sender` and `Recieve` types for sending and receiving data.
- **Thread-Safe**: Leveraging Rust's `Atomic` types and synchronization primitives, the implementation ensures safety across threads.
- **Custom Lock-Free Design**: The implementation uses a spinlock mechanism for synchronization to avoid expensive OS-level locks.

## Usage

### Creating a Channel
You can create a new channel using the `new` function:
```rust
use mpsc::new;

let (sender, reciever) = new();
