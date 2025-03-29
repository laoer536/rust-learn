//迭代器模式：对一系列项执行某些任务
//迭代器负责：遍历每个项，确定序列（遍历）何时完成
//Rust的迭代器：
//懒惰的：除非调用消费迭代器的方法，否则迭代器本身没有任何效果
fn main() {
    println!("Hello, world!");
}

fn demo1() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); //如果后续没有使用的话 迭代器本身没有任何效果
    // for val in v1_iter { //for循环取得了v1_iter的所有权 内部以及将其变为可变的了
    //     println!("Got: {}", val);
    // }
}

//所有迭代器都实现了Iterator trait 定义于标准库 大致内容如下
// pub trait Iterator {
//     type Item; //type Item和Self::Item定义了与该trait关联的类型 实现Iterator trait需要你定义一个Item的类型，它用于next方法的返回类型（迭代器的返回类型）
//     fn next(&mut self) -> Option<Self::Item>; //Iterator trait仅要求实现一个方法 next:每次返回迭代器其中的一项 返回结果 包裹在Some里，迭代结束，返回None.可直接在迭代器上调用next方法
//     //methods with default implementations elided
// }
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect() //into_iter创建的迭代器会获得所有权
}

struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
//自定义迭代器 从1到5
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Counter, Shoe, shoes_in_my_size};

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter(); //iter方法：在不可变引用上创建迭代器 into_iter创建的迭代器会获得所有权
        // let mut v1_iter = v1.into_iter(); //into_iter创建的迭代器会获得所有权
        // let mut v1_iter = v1.iter_mut(); //iter_mut可迭代可变引用 这里v1是不可变引用 所以不能用
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
    }
    //调用next方法叫做消耗型适配器，因为调用它们会把迭代器消耗尽，例如
    //sum方法：取得迭代器所有权，通过反复调用next,遍历所有元素，每次迭代，把当前元素添加到一个总和里，迭代结束，返回总和
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);
    }
    //map接收一个闭包，闭包作用于每个元素，产生一个新的迭代器
    #[test]
    fn iterator_map() {
        let v1 = vec![1, 2, 3];
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); //collect方法：消耗型适配其，把结果收集到一个集合类型中。
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];
        let in_my_size = shoes_in_my_size(shoes, 10);
        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                }
            ]
        )
    }
    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();
        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }
    #[test]
    fn using_other_iterator_trait_methods() {
        //零开销抽象，使用抽象方法时（这里指iter相关的）不会引入额外的运行时开销 并不会因为抽象的语法增加额外的运行时性能损耗 和普通的遍历（for遍历）操作性能是差不多的 甚至抽象迭代器会更快一点  （内部实现有一个展开策略的概念 rust提前做了优化）
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a + b)
            .sum();
        assert_eq!(18, sum);
    }
}
