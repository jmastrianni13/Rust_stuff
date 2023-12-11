use std::sync;
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: sync::mpsc::Sender<Job>,
}

impl ThreadPool {
    // Create a new ThreadPool.
    //
    // The size is the number of threads in the pool.
    //
    // # Panics
    //
    // The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = sync::mpsc::channel();

        let receiver = sync::Arc::new(sync::Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, sync::Arc::clone(&receiver)));
        }

        return ThreadPool { workers, sender };
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

struct Worker {
    id: usize,
    w_thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: sync::Arc<sync::Mutex<sync::mpsc::Receiver<Job>>>) -> Worker {
        let w_thread = thread::spawn(|| {
            receiver;
        });

        return Worker { id, w_thread };
    }
}

struct Job;
