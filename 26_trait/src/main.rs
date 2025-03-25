/*
Trait的定义 将不同的方法签名组成到一个方法签名中

 */
// 定义一个trait

pub trait Summary {
    fn summarize(&self) -> String;
}

// 定义一个结构体
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 为NewsArticle实现Summary trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// 另一个结构体
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// 为Tweet实现Summary trait
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    // 创建新闻文章实例
    let article = NewsArticle {
        headline: String::from("Rust 1.75.0 发布"),
        location: String::from("北京"),
        author: String::from("张三"),
        content: String::from("Rust 1.75.0 带来了许多激动人心的新特性..."),
    };

    // 创建推文实例
    let tweet = Tweet {
        username: String::from("rust_lang"),
        content: String::from("Rust 1.75.0 现已发布！"),
        reply: false,
        retweet: true,
    };

    // 调用summarize方法
    println!("新闻摘要: {}", article.summarize());
    println!("推文摘要: {}", tweet.summarize());
} 