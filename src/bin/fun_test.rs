fn info(str:String){//无返回值
    println!("{}",str);
}
fn add(a: i32, b: i32) -> i32 {//返回类型为i32
    return a + b;
}
fn add2(a: i32, b: i32) -> i32 {//返回类型为i32
    a + b //不写return的写法，最后一行即为返回值，注意结尾不能加分号
}
fn main() {
    println!("{}", add(1, 2));
    info("str".to_string());
    //代码块
    let a = {
        let b = 4;
        b + 1
    };
    println!("{}", a);

    //嵌套函数
    fn add3(a: i32, b: i32) -> i32 {
        return a + b;
    }
    println!("{}", add3(1, 2));
}