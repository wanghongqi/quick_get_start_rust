fn main(){
    let str:String=String::from("test");
    let str2:String="你好".to_string();
    let str3:String=1.to_string();
    let str4:&str="123";
    let str5:&str=str2.as_ref();
    let str5:&str=&str2;
    println!("{}",str5);//输出 你好
    let str6:String=String::from(str4);
    let str7:String=str4.to_string();
    println!("{}",str7);//输出 123
    //字符串连接，需要String+&str+&str这种形式，只有第一个是String，这种会把第一个字符串变量move走，不如下面的format!宏方便
    let str8:String=str + str2.as_ref() +"你好";
    let str:String=String::from("test");
    let str9:String=str + &str2 +"你好";
    println!("{}",str9);//输出 test你好你好
    //字符串使用format!宏连接，多个参数可以都是字符串
    let str:String=String::from("test");
    let s:String= format!("{}{}{}",str,str2,str3);
    println!("{}",s);//输出 test你好1

    //字符串追加
    let mut str:String=String::from("hello");
    str.push_str("sss");//追加字符串
    str.push('!');//追加字符
    println!("{}",str);//输出 hellosss!

    //字符串长度（String与&str相同）
    let str:&str="hello";
    let str2:&str="你好";
    let len:usize=str.len();
    println!("{}",len);
    let len2:usize=str2.len();//你好，UTF-8字符集中中文是三个字节，所以这里字节长度为6
    println!("{}",len2);
    /*输出
    5
    6
    */
    let str2:&str="你好";
    let len3:usize=str2.chars().count();//按字符方式统计，为2个字符
    println!("{}",len3);//输出为2
    let str2:String=String::from("你好");
    let len2:usize=str2.len();
    println!("{}",len2);
    let len3:usize=str2.chars().count();
    println!("{}",len3);
    /**输出
    6
    2
    **/

    //取单个字符（String与&str相同）
    let str:String=String::from("你好China");
    let a:Option<char>=str.chars().nth(1);
    println!("{:?}",a);
    let a:Option<char>=str.chars().nth(2);
    println!("{:?}",a);
    /**输出
    Some('好')
    Some('C')
    **/

    //字符串直接切割
    let str:String=String::from("你好China");
    let sub=&str[0..3];
    println!("{}",sub);
    //输出 ：你
}