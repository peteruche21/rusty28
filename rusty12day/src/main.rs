use std::sync::{mpsc, Arc, Mutex};
use std::thread;

mod rayon;
use rayon::rayon;

mod assignment;
use assignment::assignment;

fn main() {
    let handle = thread::spawn(move || {
        println!("Hello from thread!");
    });

    handle.join().unwrap();

    let v = vec![1, 2, 3];

    // let handle2 = thread::spawn(move || {
    //     println!("{:?}", v);
    // });

    let mut thread_handles = Vec::new();

    for e in v {
        thread_handles.push(thread::spawn(move || println!("Thread {}", e)));
    }
    println!("hello from main");

    for handle in thread_handles {
        handle.join().unwrap();
    }

    let (transmitter, receiver) = mpsc::channel();
    let tx = transmitter.clone();
    thread::spawn(move || {
        let vec = vec![
            String::from("clone"),
            String::from("is"),
            String::from("transmitting"),
        ];
        for val in vec {
            transmitter.send(val).unwrap();
        }
    });
    thread::spawn(move || {
        let vec = vec![
            String::from("clone"),
            String::from("is"),
            String::from("transmitting"),
        ];
        for val in vec {
            tx.send(val).unwrap();
        }
    });

    for rec in receiver {
        println!("{}", rec);
    }

    // let val = String::from("Transmitting");

    // thread::spawn(move || {
    //     transmitter.send(val).unwrap();
    // });

    // let msg = receiver.recv().unwrap();
    // println!("{}", msg);

    let rc1 = Arc::new(String::from("Test"));
    let rc2 = rc1.clone();

    thread::spawn(move || drop(rc2));

    let counter = Arc::new(Mutex::new(0));
    let mut handlex = vec![];

    for _ in 0..8 {
        let counter = Arc::clone(&counter);
        let handl = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handlex.push(handl);
    }
    for handl in handlex {
        handl.join().unwrap();
    }

    println!("{}", counter.lock().unwrap());

    rayon();

    assignment();
}
