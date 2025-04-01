//Trait对象执行的是动态派发
//将trait约束作用于范型时，Rust编译器会执行单态化：编译器会为我们用来替换范型类型参数的每一个具体类型生成对应函数和方法的非泛型实现。
//通过单态化生成的代码会执行静态派发（static dispatch），在编译过程中确定调用的具体方法
//动态派发（dynamic dispatch）: 无法在编译过程中确定你调用的究竟是哪一种方法，编译器会产生额外的代码以便在运行时找出希望调用的方法
//使用trait对象，会执行动态派发： 产生运行时开销 阻止编译器内联方法代码，使得部分优化操作无法进行

//Trait对象必须保证对象安全 只能把满足对象安全的trait转化为trait对象
//Rust采用一系列规则来判定某个对象是否安全，只需记住两条：1、方法的返回类型不是Self 2、方法中不包含任何泛型类型参数

//静态派发demo
//trait Draw {
//     fn draw(&self);
// }
//
// struct Circle;
// struct Square;
//
// impl Draw for Circle {
//     fn draw(&self) { println!("Drawing a circle"); }
// }
//
// impl Draw for Square {
//     fn draw(&self) { println!("Drawing a square"); }
// }
//
// // 静态派发：通过泛型约束
// fn render_static<T: Draw>(shape: T) {
//     shape.draw();
// }
//
// fn main() {
//     let circle = Circle;
//     let square = Square;
//
//     render_static(circle); // 编译期生成 `render_static::<Circle>`
//     render_static(square); // 编译期生成 `render_static::<Square>`
// }

//动态派发demo
//trait Draw {
//     fn draw(&self);
// }
//
// struct Circle;
// struct Square;
//
// impl Draw for Circle {
//     fn draw(&self) { println!("Drawing a circle"); }
// }
//
// impl Draw for Square {
//     fn draw(&self) { println!("Drawing a square"); }
// }
//
// // 动态派发：通过 Trait 对象
// fn render_dynamic(shape: &dyn Draw) {
//     shape.draw();
// }
//
// fn main() {
//     let shapes: Vec<Box<dyn Draw>> = vec![
//         Box::new(Circle),
//         Box::new(Square),
//     ];
//
//     for shape in shapes {
//         render_dynamic(shape.as_ref()); // 运行时动态调用
//     }
// }

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, //Box<dyn Draw> 表示Box这个里面的元素都是实现了Draw这个trait的
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// ------------- 范型实现
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }
//
// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

pub struct Button {
    pub label: String,
    pub width: u32,
    pub height: u32,
}

impl Draw for Button {
    fn draw(&self) {
        //绘制一个按钮
    }
}

pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        //绘制一个选择框
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();
}
