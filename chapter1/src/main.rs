#![feature(thread_id_value)]

use std::sync::atomic::{AtomicI32, Ordering};
use std::{sync::Arc, sync::Mutex, thread};

fn main() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);
    let t3 = thread::spawn(f);

    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();

    let numbers = vec![1, 2, 3];
    thread::spawn(move || {
        // move: move the ownership of numbers to the thread
        for n in numbers {
            print!("{n} ");
        }
        println!();
    })
    .join()
    .unwrap();

    let numbers = Vec::from_iter(0..1000);
    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.into_iter().sum::<usize>(); // into_iter: consume the vector
        sum / len
    });
    let average = t.join().unwrap();
    println!("average is {average}");

    // leaking
    let x: &'static [i32; 3] = Box::leak(Box::new([4, 5, 6]));
    let t1 = thread::spawn(move || println!("{:?}", x));
    let t2 = thread::spawn(move || println!("{:?}", x));
    t1.join().unwrap();
    t2.join().unwrap();

    // arc
    let x = Arc::new(AtomicI32::new(0));
    let x1 = x.clone();
    let x2 = x.clone();
    let t1 = thread::spawn(move || {
        for _ in 0..100000 {
            let val = x1.load(Ordering::Acquire);
            x1.store(val + 1, Ordering::Release);
        }
    });
    let t2 = thread::spawn(move || {
        for _ in 0..100000 {
            let val = x2.load(Ordering::Acquire);
            x2.store(val - 1, Ordering::Release);
        }
    });
    t1.join().unwrap();
    t2.join().unwrap();
    println!(
        "final value: {}, possibly not 0 :(",
        x.load(Ordering::Acquire)
    );
    // arc + mutex
    let x = Arc::new(Mutex::new(0));
    let t1 = thread::spawn({
        let x = x.clone(); // shadow the x :)
        move || {
            let mut x = x.lock().unwrap();
            *x += 1;
        }
    });
    let t2 = thread::spawn({
        let x = x.clone();
        move || {
            let mut x = x.lock().unwrap();
            *x -= 1;
        }
    });
    t1.join().unwrap();
    t2.join().unwrap();
    assert_eq!(*x.lock().unwrap(), 0);
    println!(
        "final value: {}, should be 0 or your computer is broken :O",
        x.lock().unwrap()
    );
}

fn f() {
    println!("hello from wheatfox :)");
    let id = thread::current().id();
    println!("my thread id is {:?}", id.as_u64());
}
