use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    // run these individually for cleaner results
    demo_threads();
    demo_move();
    demo_mpsc();
    demo_mutex();
    demo_mutex_wthreads()
}

fn demo_threads() {

    let blockinghandle = thread::spawn(|| {
        for i in 1..4 {
            println!("hi number {i} from the blocking joinhandle thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    blockinghandle.join().unwrap(); // will block any threads started below this line

    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1000));
        }
    });

    let handle = thread::spawn(|| {
        for i in 1..8 {
            println!("hi number {i} from the joinhandle thread!");
            thread::sleep(Duration::from_millis(750));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(500));
    }

    handle.join().unwrap() // main thread will wait for this thread to finish
}

fn demo_move() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

fn demo_mpsc() {
    // mpsc stands for multi producer, single consumer
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for msg in rx {
        println!("Got: {}", msg);
    };

}

fn demo_mutex() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6
    }

    println!("m = {:?}", m);
}

fn demo_mutex_wthreads () {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

