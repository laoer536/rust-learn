use hello_macro::HelloMacro;
struct Pancakes;
impl HelloMacro for Pancakes { //传统写法 这样的话如果我很多地方都要使用到HelloMacro这个trait 那么我都得实现一次 就很麻烦 考虑使用过程宏来自动生成相应的代码
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}
fn main() {
    Pancakes::hello_macro();
}

// use hello_macro::HelloMacro;
// use hello_macro_derive::HelloMacro;

// #[derive(HelloMacro)]
// struct Pancakes;

//fn main(){
//  Pancakes::hello_macro();
// }
