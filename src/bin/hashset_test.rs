use std::collections::HashSet;

fn main(){
    //声明HashSet HashSet中的值不能重复
    let mut hs=HashSet::new();
    //插入值
    hs.insert(String::from("whq"));
    hs.insert(String::from("test"));
    hs.insert(String::from("test"));//插入失败但不会引发异常
    //输出
    println!("{:?}",hs);//输出 {"test", "whq"}
    //获取长度
    println!("len:{}",hs.len());//输出 len:2
    //迭代器遍历
    for s in hs.iter(){
        println!("{}",s);
    }
    /**输出
    test
    whq
    **/

    //get获取值是否存在
    match hs.get(&String::from("whq")){
        Some(value)=>{
            println!("found {}",value);
        }
        None =>{
            println!("not found");
        }
    }
    //输出  found whq

    //判断值是否存在
    println!("{}",hs.contains(&String::from("haha"))); //输出 false
    //删除值
    println!("{}",hs.remove(&String::from("haha"))); //输出 false
    println!("{}",hs.remove(&String::from("test"))); //输出 true
    println!("{:?}",hs);//输出  {"whq"}
}