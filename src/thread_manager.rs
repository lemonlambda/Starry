

use std::thread::Builder as ThreadBuilder;
use std::sync::{Arc, Mutex};

use std::thread::available_parallelism;

pub struct ThreadManager {
    threads: Vec<Arc<Mutex<ThreadState>>>
}

pub struct ThreadState {
    thread: ThreadBuilder,
    finished: bool,
}

impl ThreadManager {
    pub fn new() -> Self {
        Self {
            threads: create_max_threads()
        }
    }

    /// This will busy wait if no thread is available
    pub fn get_available_thread(&mut self) -> Arc<Mutex<ThreadState>> {
        let mut available;
        loop {
            for thread in &self.threads {
                match thread.try_lock() {
                    Ok(_) => {
                        available = thread;
                        break;
                    }
                    Err(_) => { continue; }
                };
            }
        }
        available.clone()
    }
}

fn create_max_threads() -> Vec<Arc<Mutex<ThreadState>>> {
    let mut threads = vec![];
    for _ in 0..available_parallelism().unwrap().into() {
        threads.push(Arc::new(Mutex::new(ThreadState {
            thread: ThreadBuilder::new(),
            finished: false
        })));
    }
    threads
}