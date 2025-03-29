//二进制程序关注点分离的指导性原则：
//1、将程序拆分为main.rs和lib.rs,将业务逻辑放入lib.rs
//2、当命令行解析逻辑较少时，将它放在main.rs
//3、当命令行解析逻辑变复杂时，需要将它从main.rs提取到lib.rs
use command_line_tool::{Config, run};
use std::{env, process};

//标准输出：stdout -> println!
//标准错误：stderr -> eprintln!

//在命令行环境中（如 bash、cmd、PowerShell 等），符号 > 是 输出重定向操作符，它的作用是将命令的标准输出（stdout）重定向到指定文件（覆盖文件原有内容）。这与 cargo 无关，而是 Shell 的功能。 例如：cargo build > build.log
//> 仅重定向标准输出（stdout）。若需重定向错误信息（stderr），需使用 2>： 例如cargo build 2> error.log
//若想同时重定向 stdout 和 stderr，可以使用 &>（在 bash 中）： cargo build &> all_output.log

fn main() {
    let args = env::args(); //获取命令行参数集合
    let config = Config::new(args).unwrap_or_else(|err| {
        //这里类似TS回调函数 err是函数参数 {}里面是函数体
        eprintln!("Problem parsing arguments: {}", err); //cargo run > output.txt
        process::exit(1);
    });
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e); //cargo run cargo README.md > output.txt
        process::exit(1);
    };
}
