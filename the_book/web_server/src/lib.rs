use std::sync;
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<sync::mpsc::Sender<Job>>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("shutting down worker {}", worker.id);

            if let Some(w_thread) = worker.w_thread.take() {
                w_thread.join().unwrap();
            }
        }
    }
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

        return ThreadPool {
            workers,
            sender: Some(sender),
        };
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    w_thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: sync::Arc<sync::Mutex<sync::mpsc::Receiver<Job>>>) -> Worker {
        let w_thread = thread::spawn(move || loop {
            let msg = receiver.lock().unwrap().recv();

            match msg {
                Ok(job) => {
                    println!("> worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    println!("> worker {id} disconnected; shutting down");
                    break;
                }
            }
        });

        return Worker {
            id,
            w_thread: Some(w_thread),
        };
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;
