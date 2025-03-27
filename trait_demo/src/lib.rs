//Trait告诉Rust编译器：某种类型具有哪些并且可以与其他类型共享的功能

use std::fmt::{Debug, Display};

//trait定义行为
pub trait Summary {
    fn summarize(&self) -> String; //只是定义 需要impl实现
    fn summarize1(&self) -> String; //只是定义 需要impl实现
    //是定义+实现 称为默认实现 就不需要强制impl实现了 当然也可以重写
    fn summarize2(&self) -> String {
        String::from("(Read more...)")
    }

    fn summarizes_author(&self) -> String;

    //这样直接使用声明 那么如果要使用这个方法 就需要impl实现summarizes_author函数
    fn summarize3(&self) -> String {
        format!("(Write more from {}...)", self.summarizes_author())
    }
}

//struct定义数据
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    //因为Trait定义了行为 所以这里都要实现
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn summarize1(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    //默认实现也可以被重写
    fn summarize2(&self) -> String {
        format!("{} ({})", self.headline, self.author)
    }

    //由于Trait summarize3的原因 那么需要具体实现 summarizes_author
    fn summarizes_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    //因为Trait定义了行为 所以这里都要实现
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize1(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    //由于Trait summarize3的原因 需要具体实现 summarizes_author
    fn summarizes_author(&self) -> String {
        format!("{}", self.username)
    }
}

//在使用泛型类型参数的impl块上使用Trait bound，我们可以有条件地为特定的Trait实现方法
//impl<T: fmt::Display + ?Sized> ToString for T {
//     #[inline]
//     fn to_string(&self) -> String {
//         <Self as SpecToString>::spec_to_string(self)
//     }
// }

//Trait作为参数
//impl Trait 语法:适用于简单情况
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound语法：可用于复杂情况
pub fn notify2<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

//使用+号指定多个Trait
pub fn notify3(item: impl Summary + Display) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify4<T: Summary + Display>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify5<T: Summary + Display, U: Clone + Debug>(a: T, b: U) -> String {
    format!("Breaking news! {}", a.summarize())
}

//使用where指定范型具体Trait（在函数签名之后） 来避免参数处太长 并且更加直观
pub fn notify6<T, U>(a: T, b: U) -> String
where
    T: Display + Summary,
    U: Clone + Debug,
{
    format!("Breaking news! {}", a.summarize())
}

//使用Trait作为返回值
pub fn notify7(s: String) -> impl Summary {
    NewsArticle {
        headline: s,
        content: String::from(""),
        author: String::from(""),
        location: String::from(""),
    }
}

//注意：只能返回确定的同一种类型，返回可能不同类型的代码会报错
pub fn notify8(flag: bool) -> impl Summary {
    // if flag {
    //     NewsArticle {
    //         headline: String::from(""),
    //         content: String::from(""),
    //         author: String::from(""),
    //         location: String::from(""),
    //     }
    // } else {
    //     //Tweet Error
    //     Tweet {
    //         username: "".to_string(),
    //         content: "".to_string(),
    //         reply: false,
    //         retweet: false,
    //     }
    // }
    NewsArticle {
        headline: String::from(""),
        content: String::from(""),
        author: String::from(""),
        location: String::from(""),
    }
}
