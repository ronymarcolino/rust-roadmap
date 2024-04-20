use std::{thread, time};

fn main() {
    let thousand_millis = time::Duration::from_millis(1000);
    let thread_join_handle = thread::spawn(move || {
        // some work here
        let mut counter = 0;
        loop{
            println!("Thread 1 {}", counter);
            //let now = time::Instant::now();
            counter += 1;
            if counter >= 5 {
                break;
            }
            thread::sleep(thousand_millis);
        }
    });
    let res = thread_join_handle.join();
}
