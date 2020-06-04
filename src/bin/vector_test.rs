fn main(){
    let mut vector: Vec<i32> = Vec::new(); // 创建类型为 i32 的可变空向量
    vector.push(1);//向量中添加值
    vector.push(2);
    println!("{:?}",vector);//打印整个向量
    let mut vector = vec![1, 2, 3];     // 通过数组创建可变向量
    vector.append(&mut vec![4,5,6]);//向量尾部添加另外一个向量
    println!("{:?}",vector);
    //获取向量中的一个值，由于向量长度不固定，可能有返回None的情况，所以返回的是Option，需要match处理。
    println!("{}",match vector.get(0) {
        Some(value)=>value.to_string(),
        None=>"None".to_string()
    });
    println!("{}",vector[0]);//这种数组取值的方式可以直接取，但是如果获取为空的话会直接报错
    /*输出
    [1, 2]
    [1, 2, 3, 4, 5, 6]
    1
    */

    //遍历过程中修改值
    let mut vector=vec![100, 32, 57];
    for i in &mut vector {
        *i += 50;
    }
    println!("{:?}",vector);
    //[150, 82, 107]
}