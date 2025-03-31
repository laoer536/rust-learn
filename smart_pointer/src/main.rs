//指针（引用）&
//指针和智能指针的区别
//引用只借用数据 智能指针很多时候都拥有它所指向的数据
//例如 String Vec<T> 就是智能指针 （都拥有一片内存区域，且允许用户对其操作，还拥有原数据（例如容量等），提供额外的功能或保障（String保障其数据都是合法的UTF-8编码））

//智能指针通常使用struct实现，并且实现了Deref和Drop这两个trait
//Deref trait: 允许智能指针struct的实例像引用一样使用 使可以自定义解引用运算符*的行为 使智能指针可像常规引用一样来处理
//Drop trait: 允许你自定义当智能指针实例走出作用域时的代码 当值要离开作用域时发生的动作

//Box<T>: 在head内存上分配值（堆内存）stack上是指向heap上储存的数据 没有性能开销 没有其他额外功能 一个 支持可变和不可变借用（编译时检查）
//Rc<T>: 启用多重所有权的引用计数类型（引用计数，类似JS中的引用计数） 有时，一个值会有多个所有者，为了支持多重所有权 只能用于单线程场景 多个 不可变借用（编译时检查）
//Ref<T>和RefMut<T>,通过RefCell<T>访问：在运行时而不是编译时强制借用规则的类型 一个 可变、不可变借用（运行时检查）
//扩展：
//Cell<T>:通过复制来访问数据
//Mutex<T>:用于实现跨线程情形的内部可变性模式

//此外：
//内部可变模式：不可变类型暴露出可修改其内部值的API

use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::rc::Rc;

//引用循环：它们如何泄露，以及如何防止其发生。
fn main() {
    let b = Box::new(5);
    println!("{}", b);
    let list = Cons(
        1,
        Box::new(Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Nil))))))),
    );
    drop_custom_smart()
} //执行完毕后 Box 指针（在栈）和指向的数据（在堆）都会被清理

enum List {
    // Cons(i32, List),
    Cons(i32, Box<List>), //用指针来存放
    Nil,
}

//Rust需要确定分配空间大小 如下就是采用所需空间最大的那一个来作为需要分配的空间
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn dequote_demo() {
    let x = 5;
    let y = &x;
    let z = Box::new(x);
    let m = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y); //解引用 解开的*y就是具体的值
    assert_eq!(5, *z); //解引用 解开的*z就是具体的值
    assert_eq!(5, *m); //解引用 解开的*m就是具体的值 内部相当于 *(m.deref());

    let n = MyBox::new(String::from("Rust"));
    //Rust在编译阶段会做如下操作：
    //&n &MyBox<String>
    //deref &String
    //deref &str
    hello(&n);
    hello("Rust");
}

struct MyBox<T>(T); //只有一个成员的元组
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        //这样就可以解引用了
        &self.0
    }
}

fn hello(name: &str) {
    println!("{}", "hello");
}

struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    // impl xx for yy  xx就是trait yy就是结构体struct
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data); //离开作用域时（清理时）调用
    }
}
fn drop_custom_smart() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    // c.drop();//Error 不允许手动调用Drop trait的drop方法
    drop(c); //这样可以 可以使用std::mem::drop函数，来提前drop值
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
