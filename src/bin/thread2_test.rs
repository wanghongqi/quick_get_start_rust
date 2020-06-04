use std::thread;
use std::time::Duration;
fn main() {
    let handle= thread::spawn(|| {
        for i in 1..5 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();//等待线程handle运行结束
}