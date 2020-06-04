pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
fn main() {
    let article = NewsArticle { headline:String::from("head"),location:String::from("http://a/b"),author:String::from("whq"), content: String::from("content") };
    println!("{}", article.summarize());
}
//输出 (Read more...) 可见走的是默认特性summarize实现