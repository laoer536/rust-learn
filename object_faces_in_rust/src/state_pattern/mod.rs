//状态模式(state pattern)是一种面向对象设计模式：
//一个值拥有的内部状态由数个状态对象（state object）表达而成，而值的行为则随着内部状态的改变而改变
//使用状态模式意味着：业务需求变化时，不需要修改持有状态值的代码，或者使用这个值的代码，只需要更新状态对象内部的代码，以便改变其规则。或者增加一些新的状态对象。

//缺点：
//某些状态之间是相互耦合的
//需要重复实现一些逻辑代码

// trait State {
//     fn request_review(self: Box<Self>) -> Box<dyn State>;
//     fn approve(self: Box<Self>) -> Box<dyn State>;
//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         ""
//     }
// }
// struct Post {
//     state: Option<Box<dyn State>>,
//     content: String,
// }
//
// impl Post {
//     pub fn new() -> Post {
//         Post {
//             state: Some(Box::new(Draft {})),
//             content: String::new(),
//         }
//     }
//
//     pub fn add_text(&mut self, text: &str) {
//         self.content.push_str(text);
//     }
//
//     pub fn content(&self) -> &str {
//         self.state.as_ref().unwrap().content(self)
//     }
//
//     pub fn request_review(&mut self) {
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.request_review());
//         }
//     }
//     pub fn approve(&mut self) {
//         if let Some(s) = self.state.take() {
//             self.state = Some(s.approve());
//         }
//     }
// }
//
// struct PendingReview {}
// impl State for PendingReview {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         Box::new(Published {})
//     }
// }
//
// struct Draft {}
// impl State for Draft {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         Box::new(PendingReview {})
//     }
//
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
// }
//
// struct Published {}
// impl State for Published {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     fn approve(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
//     fn content<'a>(&self, post: &'a Post) -> &'a str {
//         &post.content
//     }
// }

use crate::AveragedCollection;
use std::future::Pending;

//优化：将状态和行为编码为类型(相应的状态下只能做相应的时 而不是每状态都去实现一套)
//面向对象的经典模式并不总是Rust编程实践中最佳的选择，因为Rust具有所有权等其它面向对象语言没有的特性。rust不仅能实现面向对象的设计模式，还可以支持更过的模式。
pub struct Post {
    content: String,
}
pub struct DraftPost {
    content: String,
}
pub struct PendingReviewPost {
    content: String,
}
impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }
    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}

fn main() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    let post = post.request_review();
    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
