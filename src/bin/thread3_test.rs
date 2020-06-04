use std::thread;
use std::time::Duration;
fn main() {
    //move强制所有权转移
    let v = vec![1, 2, 3];
    let handle = thread::spawn(|| {
        //println!("Here's a vector: {:?}", v);//报错：closure may outlive the current function, but it borrows `v`, which is owned by the current function
    });
    handle.join().unwrap();

    //move关键字
    let v = vec![1, 2, 3];
    let a = 1;
    let handle = thread::spawn(move|| {
        println!("Here's a vector: {:?}", v);
        println!("Here's a i32: {}", a);
    });
    handle.join().unwrap();
    //println!("{:?}",v);  //这行代码会报错，因为v已经被转移了
    println!("{}",a);  //这行代码不会报错

}