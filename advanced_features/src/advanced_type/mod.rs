use std::fmt;
use std::io::Error;

//使用类型别名创建类型同义词
//Rust提供了类型别名的功能：为现有类型生产另外的名称（同义词） 并不是一个独立的类型 使用type关键字 减少代码重复
type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;
fn main() {
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x+y={}", x + y);

    // let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));//pre
    let f: Thunk = Box::new(|| println!("hi")); //now
}

// fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) { //pre
//     //
// }
fn takes_long_type(f: Thunk) { //now
    //
}

// fn returns_long_type() -> Box<dyn Fn() + Send + 'static> { //pre
//     Box::new(|| println!("hi"))
// }
fn returns_long_type() -> Thunk {
    //now
    Box::new(|| println!("hi"))
}

type Result<T> = std::io::Result<T>;

pub trait Write {
    // fn write(&mut self, buf: &[u8]) -> Result<usize, Error>; //pre
    fn write(&mut self, buf: &[u8]) -> Result<usize>; //now
    // fn flush(&mut self) -> Result<(), Error>;//pre
    fn flush(&mut self) -> Result<()>; //now
    // fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;//pre
    fn write_all(&mut self, buf: &[u8]) -> Result<()>; //now
    // fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;//pre
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>; //now
}

//Never类型
//有一个名为!的特殊类型
//他没有任何值，行话称为空类型（empty type）
//我们倾向于叫它never类型，因为它在不返回的函数中充当返回类型

//不返回值的函数也被称为发散函数（diverging function）
fn bar() -> ! {
    panic!("panic! 返回值也是")
} //continue返回值也是! panic! 返回值也是 无法终止的循环也是

//动态大小和Sized Trait
//Rust需要在编译时确定为一个特定类型的值分配多少空间
//动态大小的类型（Dynamically Sized Types, DST）的概念：编写代码时使用只有在运行时才能确定大小的值
//str是动态大小类型（注意不是&str）: 只有运行时才能确定字符串的长度

//下列代码无法正常工作
// fn demo() {
//     let s1: str = "Hello there";
//     let s2: str = "How's it going?";
// }
//解决办法：使用&str（字符串切片）来解决
//存的是str的地址以及str的长度

//另外每个trait都是一个动态大小类型，可以通过名称对其进行引用
//为了将trait用作trait对象，必须将它放置在某种指针之后（例如&dyn Trait 或 Box<dyn Trait> (Rc<dyn Trait>)之后）

//使用Sized Trait来确定一个类型的大小在编译时是否已知
//编译时可计算出大小的类型会自动实现这一Trait
//Rust还会为每一个泛型函数隐式的添加Sized约束
fn generic<T>(t: T) {}

//上述代码在编译时会编译成这样
// fn generic<T:Sized>(t:T){
//
// }

//默认情况下，泛型函数只能被用于编译时已经知道大小的类型，可以通过特殊语法解除这一限制
//?Sized trait约束
// fn generic<T:?Sized>(t:&T){ //表示T可能是Sized也可能不是(所以t的类型也变成了T的引用 因为不确定是不是Sized) 但这个语法只能用于Sized不能用于其他Trait
//
// }
