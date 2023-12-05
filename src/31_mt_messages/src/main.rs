use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn mt_msg_1() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let val = String::from("Hello");
        tx.send(val).unwrap();
    });
    
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn mt_msg_2() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread")
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

fn mt_msg_3() {
    let (tx, rx) = mpsc::channel();
    
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread")
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
            String::from("you")
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

fn main() {
    mt_msg_1();
    mt_msg_2();
    mt_msg_3();
}