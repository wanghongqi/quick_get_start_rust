fn main(){
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    //创建可变结构体变量
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{:?}",user1);
    println!("{:#?}",user1);
    /**输出
    User { username: "someusername123", email: "someone@example.com", sign_in_count: 1, active: true }
    User {
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
        active: true,
    }
    **/
    //赋值另外的方式
    let email=String::from("someone@example.com");
    let user1 = User {
        email,    //等同于email:email的简写
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    let user2=User{
        sign_in_count:2,
        ..user1     //..表示拷贝另外一个结构体里的值，这种写法必须修改至少一个属性才行
    };
    println!("{:?}",user2);//输出 User { username: "someusername123", email: "someone@example.com", sign_in_count: 2, active: tru
}