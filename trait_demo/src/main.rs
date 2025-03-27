use trait_demo::{Summary, Tweet}; //Summary也是要导入的 才能使用Trait其中声明的方法

fn main() {
    println!("Hello, world!");
    let tweet = Tweet {
        username: String::from(""),
        content: String::from(""),
        reply: false,
        retweet: false,
    };
    println!("{}", tweet.summarize());

    let number_str = 3.to_string(); //所有Display Trait 都实现了一个 ToString的Trait 包含了 to_string方法 所以这里可以使用to_string
}
