use std::thread;
use std::time::Duration;

fn main() {
    demo_threads();
}

fn demo_threads() {

    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1000));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(500));
    }
}
