use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    //取key为Blue对应的value值，如果没有就返回0
    println!("{}",scores.get(&String::from("Blue")).unwrap_or(&0));
    //取key为Red对应的value值，如果没有就返回0
    println!("{}",scores.get(&String::from("Red")).unwrap_or(&0));
    //遍历输出scores这个HashMap，迭代元素是表示键值对的元组
    for p in scores.iter(){
        println!("{:?}",p);
    }
    //不存在key就插入key并赋值，如果存在该key就跳过
    scores.entry(String::from("Red")).or_insert(100);
    scores.entry(String::from("Blue")).or_insert(10);
    println!("{:?}",scores);
    //已经确定某个键的情况下直接修改值，将Red改为200
    if let Some(x)=scores.get_mut(&String::from("Red")){
        *x=200;
    }
    println!("{:?}",scores);
    //向量合并为HashMap
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores2: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("{:?}",scores2);
}
