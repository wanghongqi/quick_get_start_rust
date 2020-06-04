fn main(){
    //变量范围
    {
        // 在声明以前，变量 s 无效
        let s = "hello";
        // 这里是变量 s 的可用范围
    }
    // 变量范围已经结束，变量 s 无效
    //println!("{}",s); 由于变量s无效，这行代码会报错 cannot find value `s` in this scope

    //变量和数据交互方式：复制
    let a=4;
    let b=a;//基本数据类型，会将值进行复制
    println!("{}-{}",a,b);

    //变量和数据交互方式：移动
    let s1 = String::from("hello");
    let s2 = s1;
    //println!("{}-{}",s1,s2); //报错 borrow of moved value: `s1`

    //变量和数据交互方式：克隆
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}-{}",s1,s2);//不报错了，s1、s2分别有自己的
}