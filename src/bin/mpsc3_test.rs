use std::sync::mpsc;
use std::thread;
#[derive(Debug)]
pub struct Student {
    id: u32
}
fn main() {
    let (tx, rx): (mpsc::Sender<Student>, mpsc::Receiver<Student>) =
        mpsc::channel();
    thread::spawn(move || {
        tx.send(Student{id:1}).unwrap();
    });
    println!("receive {:?}", rx.recv().unwrap());  //正常输出 receive Student { id: 1 }
}