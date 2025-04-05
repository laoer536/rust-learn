use hello_macro_derive::HelloMacro;//生成实际代码
use hello_macro::HelloMacro;//处理Trait声明

#[derive(HelloMacro)]
struct Pancakes; //#[derive(HelloMacro)]会自动为Pancakes生成HelloMacro这个trait
fn main() {
    Pancakes::hello_macro(); //输出：Hello, Macro! My name is Pancakes!
}
