#![feature(thread_id_value)]

mod channel;
mod spinlock;

use channel::Channel;
use spinlock::SpinLock;
use std::sync::Arc;
use std::thread;

fn main() {
    let lock = Arc::new(SpinLock::new(0));
    let num_threads = 100;
    let num_loops = 10000;

    let mut handles = Vec::new();
    for _ in 0..num_threads {
        let lock = Arc::clone(&lock);
        handles.push(std::thread::spawn(move || {
            for _ in 0..num_loops {
                let mut num = lock.lock();
                *num += 1;
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let result = *lock.lock();
    println!("Final result: {}", result);
    assert_eq!(result, num_threads * num_loops, "SpinLock failed!");
    // testing message passing between threads
    let channel = Channel::new();
    let t = thread::current();
    thread::scope(|s| {
        s.spawn(|| {
            channel.send("hello world!");
            t.unpark();
        });
        while !channel.is_ready() {
            thread::park();
        }
        let message = channel.receive();
        println!("Received message: {}", message);
        assert_eq!(message, "hello world!");
    });
}
