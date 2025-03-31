//与Rc<T>不同，RefCell<T>类型代表了其持有数据的唯一所有权。只能用于单线程场景
//### **总结表**
// | 特性                | `Box<T>`                          | `RefCell<T>`                      |
// |---------------------|-----------------------------------|-----------------------------------|
// | **核心作用**        | 堆内存分配，所有权管理           | 运行时内部可变性                 |
// | **借用检查时机**    | 编译时                            | 运行时                            |
// | **所有权**          | 唯一所有者                        | 不改变所有权                     |
// | **典型场景**        | 递归类型、大数据转移、动态分发   | 不可变结构内部修改、共享可变状态 |
// | **组合使用**        | 常与 `RefCell`、`Rc` 结合         | 常与 `Rc`、`Box` 结合
fn demo() {
    let x = 5;
    // let y = &mut x;//Error 借用不可变为可变
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message)) //borrow_mut 获取内部值的可变引用
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_message = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_message, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_message.sent_messages.borrow().len(), 1) //borrow 获取内部值的不可变引用
    }
}
