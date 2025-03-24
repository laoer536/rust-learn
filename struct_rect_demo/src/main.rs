#[derive(Debug)] //为了debug Rectangle 信息
struct Rectangle {
    width: u32,
    height: u32,
}

//impl用于给struct扩展方法 另外impl可以有多个不限制 你可以分开写也没问题 但是这里的demo没必要
impl Rectangle {
    //&self是借用 也可以是可变借用&mut self 你还可以可以获得所有权直接写"self" 这里不限制 但是相应实例的用法就不太一样了 需要考虑所有权和是否可变
    fn area(&self) -> u32 {
        self.width * self.height //self类似JS的this
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //不把self作为第一个参数的函数，这个函数叫关联函数。可以通过"::"使用这个关联函数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let s = Rectangle::square(30);
    println!("area:{:#?}", rect.area());
    println!("rect = {:#?}", rect); //后续能继续使用是因为借用
    println!("can_hold:{:#?}", rect.can_hold(&rect2)); // &rect2 -> other
    println!("square:{:#?}", s)
}
