//宏 macro
//宏在Rust里指一组相关特性的集合称谓：
//使用macro_rules!构建的声明宏
//过程宏：1、自定义#[derive]宏，用于struct或enum，可以为其指定随derive属性添加的代码 2、类似属性的宏，在任何条目上添加自定义属性 3、类似函数的宏，看起来像函数调用，对其指定为参数的token进行操作

//函数与宏的差别
//本质上，宏是用来编写可以生成其他代码的代码（元编程）
//函数在定义签名时，必须声明参数的个数和类型，宏可以处理可变的参数
//编译器会在解释代码前展开宏
//宏的定义会比函数复杂得多，难以阅读、理解和维护
//在某个文件调用宏时，必须提前定义宏或将宏引入当前作用域（函数可以在任何位置定义并在任何位置使用）

fn macro_rules_demo() {
    // let v: Vec<i32> = vec![1, 2, 3, 4, 5];
}

// macro_rules! 声明宏 写法与match相似 但匹配的是代码结构 而不是值
#[macro_export]
macro_rules! vec {
    () => (
        $crate::vec::Vec::new()
    );
    ($elem:expr; $n:expr) => (
        $crate::vec::from_elem($elem, $n)
    );
    ($($x:expr),+ $(,)?) => (
        <[_]>::into_vec(
            // This rustc_box is not required, but it produces a dramatic improvement in compile
            // time when constructing arrays with many elements.
            #[rustc_box]
            $crate::boxed::Box::new([$($x),+])
        )
    );
}

//基于属性来生成代码的过程宏
//这种形式更像函数（某种形式的过程）一些
//接收并操作输入的Rust代码
//生成另外一些Rust代码作为结果
//三种过程宏：自定义派生 属性宏 函数宏
//创建过程宏时：宏定义必须单独放在它们自己的包中，并使用特殊的包类型

// use proc_macro;
// #[some_attribute]
// pub fn some_name(input: TokenStream) -> TokenStream {}

//自定义derive宏
//需求：创建一个hello_macro包，定义一个拥有关联函数hello_macro的HelloMacro trait
//我们提供一个能自动实现trait的过程宏
//在它们的类型上标注#[derive(HelloMacro)]，进而得到hello_macro的默认实现
//demo参见 hello_macro_tool这个项目

//类似属性的宏
//与自定义derive宏类似：允许创建新的属性 但不是为derive属性生成代码
//属性宏更加灵活：derive只能用于struct和enum 而属性宏可以用于任意条目，例如函数

// #[route(GET,"/)]
// fn index(){}

// #[route(GET,"/)]过程宏的实现如下
// #[proc_macro_attribute]
// pub fn route(attr:TokenStream, item: TokenStream) -> TokenStream { //attr对应route那里传递的参数 item对应的是下面实现的函数体(demo中是 fn index)

//类似函数的宏
//函数宏定义类似于函数调用的宏，但比普通函数更加灵活
//函数宏可以接收TokenStream作为参数
//与另外两种过程宏一样，在定义中使用Rust代码来操作TokenStream
// let sql = sql(SELECT * FROM posts where id=1)

//实现一个可以解析sql语句的宏
// #[proc_macro]
// pub fn sql(input:TokenStream) -> TokenStream {
