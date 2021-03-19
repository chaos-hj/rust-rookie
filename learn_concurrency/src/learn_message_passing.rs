use std::sync::mpsc;
use std::thread;

pub fn sync_mpsc() {
    println!("sync_mpsc works...");
    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

pub fn muti_message() {
    println!("muti_message works...");
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep_ms(1);
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
pub fn muti_producer() {
    println!("muti_producer works...");
    let (tx, rx) = mpsc::channel();
    let another_tx = mpsc::Sender::clone(&tx);
    std::thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep_ms(1);
        }
    });

    std::thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            another_tx.send(val).unwrap();
            thread::sleep_ms(1);
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
