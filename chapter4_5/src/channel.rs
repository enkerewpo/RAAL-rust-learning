use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, Thread};

struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    in_use: AtomicBool,
    ready: AtomicBool,
    receiver_thread: Thread,
}

unsafe impl<T> Send for Channel<T> {}
unsafe impl<T> Sync for Channel<T> {}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            in_use: AtomicBool::new(false),
            ready: AtomicBool::new(false),
            receiver_thread: thread::current(),
        }
    }
    pub fn send(&self, message: T) {
        if self.in_use.swap(true, Ordering::Acquire) {
            panic!("cannot send more than one message");
        }
        unsafe { (*self.message.get()).write(message) };
        self.ready.store(true, Ordering::Release); // update the ready flag
        // finally, unpark the receiver thread
        println!(
            "this thread(id: {}) is sending a message and unparking receiver thread which is {:?}",
            self.receiver_thread.id().as_u64(),
            self.receiver_thread
        );
        self.receiver_thread.unpark();
    }
    pub fn is_ready(&self) -> bool {
        self.ready.load(Ordering::Relaxed)
    }
    pub fn receive(&self) -> T {
        if !self.ready.swap(false, Ordering::Acquire) {
            panic!("no message ready");
        }
        unsafe { (*self.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe { (*self.message.get()).assume_init_drop() };
        }
    }
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let channel = Arc::new(Channel::new());
    (
        Sender {
            channel: channel.clone(),
        },
        Receiver {
            channel,
            _no_send: PhantomData,
        },
    )
}

pub struct Sender<T> {
    channel: Arc<Channel<T>>,
}

pub struct Receiver<T> {
    channel: Arc<Channel<T>>,
    _no_send: PhantomData<*const T>, // prevent sending the receiver to another thread
}

impl<T> Receiver<T> {
    pub fn is_ready(&self) -> bool {
        self.channel.is_ready()
    }
    pub fn receive(&self) -> T {
        self.channel.receive()
    }
}

impl<T> Sender<T> {
    pub fn send(&self, message: T) {
        self.channel.send(message);
    }
}
