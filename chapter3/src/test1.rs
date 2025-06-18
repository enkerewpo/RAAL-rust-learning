use std::{
    collections::HashMap,
    sync::atomic::{AtomicI32, Ordering},
    thread,
    time::Duration,
    sync::Arc,
    sync::Barrier,
};

static X: AtomicI32 = AtomicI32::new(0);
const N: usize = 1000;

pub fn experiment1() {
    // RELAXED
    let results: [(i32, i32); N] = std::array::from_fn(|_| {
        X.store(0, Ordering::Relaxed);
        let t1 = thread::spawn(|| {
            X.store(1, Ordering::Relaxed);
            std::thread::sleep(Duration::from_nanos(1));
            X.load(Ordering::Relaxed)
        });
        let t2 = thread::spawn(|| {
            X.store(2, Ordering::Relaxed);
            std::thread::sleep(Duration::from_nanos(1));
            X.load(Ordering::Relaxed)
        });
        let r1 = t1.join().unwrap();
        let r2 = t2.join().unwrap();
        (r1, r2)
    });
    let mut result_counts = HashMap::new();
    for (x, y) in results.iter() {
        *result_counts.entry((*x, *y)).or_insert(0) += 1;
    }
    let mut result_vec: Vec<((i32, i32), usize)> = result_counts.into_iter().collect();
    result_vec.sort_by_key(|&(k, _)| k);

    // RELEASE/ACQUIRE
    let results2: [(i32, i32); N] = std::array::from_fn(|_| {
        X.store(0, Ordering::Release);
        let t1 = thread::spawn(|| {
            X.store(1, Ordering::Release);
            std::thread::sleep(Duration::from_nanos(1));
            X.load(Ordering::Acquire)
        });
        let t2 = thread::spawn(|| {
            X.store(2, Ordering::Release);
            std::thread::sleep(Duration::from_nanos(1));
            X.load(Ordering::Acquire)
        });
        let r1 = t1.join().unwrap();
        let r2 = t2.join().unwrap();
        (r1, r2)
    });
    let mut result_counts2 = HashMap::new();
    for (x, y) in results2.iter() {
        *result_counts2.entry((*x, *y)).or_insert(0) += 1;
    }
    let mut result_vec2: Vec<((i32, i32), usize)> = result_counts2.into_iter().collect();
    result_vec2.sort_by_key(|&(k, _)| k);

    // SEQ_CST
    let results3: [(i32, i32); N] = std::array::from_fn(|_| {
        X.store(0, Ordering::SeqCst);
        let t1 = thread::spawn(|| {
            X.store(1, Ordering::SeqCst);
            std::thread::sleep(Duration::from_nanos(1));
            X.load(Ordering::SeqCst)
        });
        let t2 = thread::spawn(|| {
            X.store(2, Ordering::SeqCst);
            std::thread::sleep(Duration::from_nanos(1));
            X.load(Ordering::SeqCst)
        });
        let r1 = t1.join().unwrap();
        let r2 = t2.join().unwrap();
        (r1, r2)
    });
    let mut result_counts3 = HashMap::new();
    for (x, y) in results3.iter() {
        *result_counts3.entry((*x, *y)).or_insert(0) += 1;
    }
    let mut result_vec3: Vec<((i32, i32), usize)> = result_counts3.into_iter().collect();
    result_vec3.sort_by_key(|&(k, _)| k);

    // BARRIER
    let barrier = Arc::new(Barrier::new(3));
    let results4: [(i32, i32); N] = std::array::from_fn(|_| {
        X.store(0, Ordering::Relaxed);
        let barrier1 = barrier.clone();
        let barrier2 = barrier.clone();
        let t1 = thread::spawn(move || {
            X.store(1, Ordering::Relaxed);
            barrier1.wait();
            X.load(Ordering::Relaxed)
        });
        let t2 = thread::spawn(move || {
            X.store(2, Ordering::Relaxed);
            barrier2.wait();
            X.load(Ordering::Relaxed)
        });
        barrier.wait();
        let r1 = t1.join().unwrap();
        let r2 = t2.join().unwrap();
        (r1, r2)
    });
    let mut result_counts4 = HashMap::new();
    for (x, y) in results4.iter() {
        *result_counts4.entry((*x, *y)).or_insert(0) += 1;
    }
    let mut result_vec4: Vec<((i32, i32), usize)> = result_counts4.into_iter().collect();
    result_vec4.sort_by_key(|&(k, _)| k);

    fn print_table(title: &str, results: &[((i32, i32), usize)]) {
        let total: usize = results.iter().map(|(_, c)| *c).sum();
        println!("{title}");
        for &((x, y), count) in results {
            let percent = (count as f64) * 100.0 / (total as f64);
            println!("({x}, {y}): {count} ({percent:.2}%)");
        }
        println!("Total: {total}");
    }

    print_table("MEMORY ORDERING: RELAXED", result_vec.as_slice());
    print_table("MEMORY ORDERING: RELEASE AND ACQUIRE", result_vec2.as_slice());
    print_table("MEMORY ORDERING: SEQ_CST", result_vec3.as_slice());
    print_table("MEMORY ORDERING: BARRIER", result_vec4.as_slice());
}