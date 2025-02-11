use std::thread;
use std::time::Duration;

fn main() {
    // Spawn a new thread
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Thread enfant : {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Main thread loop
    for i in 1..5 {
        println!("Thread principal : {}", i);
        thread::sleep(Duration::from_millis(150));
    }

    // Wait for the child thread to finish
    handle.join().unwrap();
}
