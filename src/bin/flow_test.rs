fn main(){
    //if常规写法
    let a = 2;
    if a == 1 {
        println!("1");
    } else if a == 2 {
        println!("2");
    } else {
        println!(">2");
    }
    //if类似三元表达式的写法
    let a = 2;
    let b = if a > 0 { 1 } else { -1 };
    println!("{}", b);

    //for各种写法
    //数组遍历
    let arr:[i32;3]=[1,2,3];
    for i in &arr {
        println!("{}",i);
    }
    /**输出
    1
    2
    3
    **/
    //数组遍历写法二
    let arr:[i32;3]=[1,2,3];
    for i in arr.iter() {//通过迭代器遍历
        println!("{}",i);
    }
    //数值遍历[0,3)
    for i in 0..3 {
        println!("{}",i);
    }
    /**输出
    0
    1
    2
    **/

    //while写法
    let mut i=1;
    while i<4 {
        println!("{}",i);
        i+=1;
    }
    /*输出
    1
    2
    3
    */
    //break跳出
    let mut i=1;
    while i<4 {
        println!("{}",i);
        i+=1;
        if i==3{
            break;
        }
    }
    /*输出
    1
    2
    */

    //loop写法
    let mut i=1;
    loop {
        println!("{}",i);
        i+=1;
        if i==3{
            break;
        }
    }
    /*输出
    1
    2
    */
}