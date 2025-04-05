//整个理解为Rust代码 -> AST -> 修改AST -> 新AST -> 新的Rust代码
extern crate proc_macro;
use crate::proc_macro::TokenStream; //proc_macro可以读取和操作Rust代码
use quote::quote; //可以将syn产生的数据结构重新转化为Rust代码
use syn;//把Rust代码字符串转为可以操作的数据结构

//#[derive(HelloMacro)] //使用时加了这句话 #[proc_macro_derive(HelloMacro)]下面的代码会自动执行 这里是hello_macro_derive
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap(); //得到AST
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let token_stream = quote! {
        impl HelloMacro for #name { //这里就是自动生成的代码 妙啊 这里#name就是使用处的struct
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    token_stream.into()
}