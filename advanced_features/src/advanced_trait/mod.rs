//在trait定义中使用关联类型来指定占位类型
//可以定义出包含某些类型的trait, 而在实现前无需知道这些类型是什么

use std::fmt;
use std::fmt::Debug;
use std::ops::Add;

pub trait Iterator {
    type Item; //Item就是类型占位符
    fn next(&mut self) -> Option<Self::Item>;
}

pub trait Iterator2<T> {
    fn next(&mut self) -> Option<T>;
}

struct Counter {}

impl Iterator for Counter {
    type Item = u32; //需要指明关联类型的指定类型 但泛型是通过手动传入
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl Iterator2<String> for Counter {
    //泛型是通过手动传入 自动替换对应位置的占位符
    fn next(&mut self) -> Option<String> {
        None
    }
}

impl Iterator2<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        None
    }
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    //Add<Rhs = Self> --->>>> Add没传泛型 默认是Self 所以这里默认是Point
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);
impl Add<Meters> for Millimeters {
    //Add<Rhs = Self> ---->>> Add传了泛型Meters 重栽了默认的Self 所以这里是Meters
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
fn test() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 4, y: 3 }
    );
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}
struct Human;
impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}
struct Dog;
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

//如何使用同名方法
fn test2() {
    let person = Human;
    person.fly(); //调用本身的方法
    Pilot::fly(&person); //调用Pilot trait定义的fly
    Wizard::fly(&person); //调用Wizard trait定义的fly

    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); //完全限定语法 为什么这么做？ 因为baby_name的定义并不像上面fly一样定义了self可以支持我们手动传入struct 这种情况下rust无法推断你要使用哪一个baby_name方法 这种情况下就要手动指定了 通过as完全限定语法 注意除非是不能推断才用 避免滥用
}

//使用supertrait来要求trait附带其他trait功能
trait OutlinePrint: fmt::Display {
    //表示OutlinePrint依赖于fmt::Display这个trait
    fn outline_print(&self) {
        let output = self.to_string(); //OutlinePrint依赖于fmt::Display这个trait 所以有to_string方法
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4))
    }
}

struct Point2 {
    x: i32,
    y: i32,
}
impl OutlinePrint for Point2 {}
impl fmt::Display for Point2 {
    //实现display这个trait
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

//使用newtype模式在外部类型上实现外部trait
//背景：孤儿规则 -->> 只有当trait或类型定义在本地包时，才能为该类型实现这个trait
//可以使用newtype模式绕过孤儿规则：利用元组结构体（tuple struct）创建一个新的类型
//如：我们想为Vector实现Display trait 但Vector和Display都定义在外部的包中 所以只有通过newtype模式来
struct Wrapper(Vec<String>); //tuple struct 相当于代理到本地定义
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn test3() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
