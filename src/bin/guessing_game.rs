use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);//生成一个[1,101)左闭右开区间的数字（即1-100）

    loop {//无限循环，类似while(true)
        println!("Please input your guess.");

        let mut guess = String::new();//创建一个字符串变量

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");//读取一行输入到字符串中

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };//转换字符串为u32（32位正整数，类似ulong），转换异常就继续下一次loop

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {//比较guess猜测的数值与生成的随机数字
            Ordering::Less => println!("Too small!"),//小了
            Ordering::Greater => println!("Too big!"),//大了
            Ordering::Equal => {//正好相等
                println!("You win!");
                break;//跳出loop结束游戏
            }
        }
    }
}