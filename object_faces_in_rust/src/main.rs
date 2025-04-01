//对象是包含数据和行为的
//基于此定义：Rust是面相对象的
//struct、enum包含数据 impl块提供方法 但带有方法的struct、enum并没有称为对象
//封装：调用对象外部的代码无法直接访问对象内部的实现细节，唯一可以与对象进行交互的方法是通过它公开的API
//继承：使对象可以沿用另外一个对象的数据和行为，且无需重复定义相关代码
//Rust是没有继承的,替代的做法是：代码复用：Rust默认trait方法来进行代码共享 多态：泛型和trait约束

//为共有行为定义一个trait(专门用于抽象某些共有行为，它没有其它语言中的对象那么通用)

mod gui;

pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn get_average(&self) -> f64 {
        self.average
    }
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

fn main() {
    println!("Hello, world!");
}
