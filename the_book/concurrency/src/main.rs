use std::thread;
use std::time::Duration;

fn main() {
    demo_threads();
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

