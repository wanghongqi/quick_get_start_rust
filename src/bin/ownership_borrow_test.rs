fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let s1 = String::from("hello");
    let mut s2 = &s1;
    let s3 = s1;
    s2=&s3;//重新从s3再租就可以了
    println!("{}", s2);
}
fn calculate_length(s: &String) -> usize {
    s.len()
}
//输出 The length of 'hello' is 5.