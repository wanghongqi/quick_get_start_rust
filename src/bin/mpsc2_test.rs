use std::thread;
use std::sync::mpsc;
fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        tx.send(String::from("I am zhangsan")).unwrap();
    });
    println!("Got: {}", rx.recv().unwrap());
    println!("Got: {}", rx.recv().unwrap());
}