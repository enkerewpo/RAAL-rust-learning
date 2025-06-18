mod test1;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;
use std::time::Duration;

static DATA: AtomicU64 = AtomicU64::new(0);
static READY: AtomicBool = AtomicBool::new(false);

fn test_release_acquire() {
    thread::spawn(|| {
        DATA.store(123, Ordering::Relaxed);
        READY.store(true, Ordering::Release);
    });
    while !READY.load(Ordering::Acquire) {
        thread::sleep(Duration::from_millis(100));
        println!("waiting...");
    }
    println!("{}", DATA.load(Ordering::Relaxed));
}

fn main() {
    test_release_acquire();
    test1::experiment1();
}
