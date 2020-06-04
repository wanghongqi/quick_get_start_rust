use std::sync::mpsc;
use std::thread;
fn main() {
    // 创建一个通道
    let (tx, rx): (mpsc::Sender<i32>, mpsc::Receiver<i32>) =
        mpsc::channel();
    // 创建线程用于发送消息
    thread::spawn(move || {
        // 发送一个消息，此处是数字id
        tx.send(1).unwrap();
    });
    // 在主线程中接收子线程发送的消息并输出
    println!("receive {}", rx.recv().unwrap());
}
//输出 receive 1