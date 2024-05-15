use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn single_thread_one_consumer_sleep() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi from thread");
        println!("Sending message {}", val);
        thread::sleep(std::time::Duration::from_millis(2000));
        println!("Trying to acess: {}", val);
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got in main thread: {}", received);
}

fn single_thread_one_consumer() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn main (){
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("1.0"),
            String::from("1.1"),
            String::from("1.2"),
            String::from("1.3"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("2.0"),
            String::from("2.1"),
            String::from("2.2"),
            String::from("2.3"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
