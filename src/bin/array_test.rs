fn main(){
    //数组定义和输出
    let arr:[i32;3]=[1,2,3];//定义数组，类型为i32,长度为3的数组，值为[1,2,3]
    println!("{:?}",arr);//输出 [1, 2, 3]

    let arr2=[1;5];//定义数组，长度为5，值都为1
    println!("{:?}",arr2);//输出 [1, 1, 1, 1, 1]

    //从数组中取值
    println!("{}",arr[0]);//输出 1
    println!("{}",arr[1]);//输出 2

    //数组单向赋值
    let mut arr=[3,4,5];
    arr[1]=1;//修改值
    println!("{:?}",arr);

    //获取数组长度
    println!("{}",arr.len());//输出 3

    //数组排序
    let mut arr:[i32;3]=[5,2,3];
    arr.sort();//排序
    println!("{:?}",arr);//输出 [2, 3, 5]

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

    //函数参数形态1 传值传递 函数对参数的修改不会影响原有数组
    println!("函数参数");
    let arr = [1,2,3];
    update(arr);
    println!("update after:{:?}",arr);//输出 update after:[1, 2, 3]
    fn update(mut arr:[i32;3]){
        for i in 0..arr.len(){
            arr[i]=arr[i]*10;
        }
        println!("update fn:{:?}",arr);//输出 update fn:[10, 20, 30]
    }

    //函数参数形态2 引用传递 函数对参数的修改影响原有数组
    let mut arr = [1,2,3];
    updateRef(&mut arr);
    println!("update after:{:?}",arr);//输出 update after:[10, 20, 30]
    fn updateRef(arr:&mut [i32;3]){
        for i in 0..arr.len(){
            arr[i]=arr[i]*10;
        }
        println!("update fn:{:?}",arr);//输出 update fn:[10, 20, 30]
    }
    println!();
}