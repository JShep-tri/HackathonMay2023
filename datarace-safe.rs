use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let counter1 = Arc::clone(&counter);
    let handle1 = thread::spawn(move || {
        for _ in 0..1_000_000 {
            let mut num = counter1.lock().unwrap();
            *num += 1;
            while *num > 0 {}
        }
    });

    let counter2 = Arc::clone(&counter);
    let handle2 = thread::spawn(move || {
        for _ in 0..1_000_000 {
            let mut num = counter2.lock().unwrap();
            *num += 1;
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let final_counter = counter.lock().unwrap();
    println!("Counter: {}", *final_counter);
}
