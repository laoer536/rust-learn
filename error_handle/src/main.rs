use std::error::Error;
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::net::IpAddr;

// 错误的分类：
// 可恢复的 例如文件未找到 可再次尝试 Result<T,E>
// 不可恢复的 bug，例如访问的索引超出范围 panic!宏(你的程序会打印一个错误信息，展开(unwind)、清理调用栈(Stack)、退出程序)
//默认 情况下 panic发生
//程序展开调用栈：Rust沿着调用栈往回走，清理每个遇到的函数中的数据。
//或立即中止调用栈：不进行清理，直接停止程序，内存需要OS进行清理。
//想让二进制更小，把设置从“展开”改为“中止”
//在Cargo.toml中适当的profile部分设置：panic = "abort"
fn main() {
    // panic!("crash and burn");
    let v = vec![1, 2, 3];
    v[99]; //Error panic! 可能出现在我们写的代码中或者我们所依赖的代码中 可通过调用panic!的函数的回溯信息来定位引起问题的代码 可设置环境变量RUST_BACKTRACE=1开获得详细的回溯信息 默认情况下 不带--release的默认启用了调试符号

    //传播错误
    let result = read_username_from_file();

    //？运算符只能用于Result、Option或者实现了Try的类型
    // let f = File::open("hello.txt")?;//Error 解决方案见read_username_from_file4
    custom_error_validation2()
}

fn result_demo() {
    //Result<T,E>
    //enum Result<T,E>{
    //  OK(T),
    //  Error(E),
    //}
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        }
    };

    // let f = File::open("hello.txt").unwrap(); //相当于上面代码的简写
    // unwrap: match表达式的一个快捷方法：
    // 如果Result结果是OK，返回OK里面的值；如果Result的结果是Err,调用panic!宏 缺点是错误信息无法自定义
    // let f = File::open("hello.txt").expect("Failed to open hello.txt"); //功能同unwrap,但是可以自定义错误信息

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                //根据不同的错误类型进行不同的操作
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });
}

fn read_username_from_file() -> Result<String, std::io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, std::io::Error> {
    //是read_username_from_file的简写 使用了？运算符
    let mut f = File::open("hello.txt")?; //？会调用from函数 转化错误 Trait std::convert::From
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, std::io::Error> {
    //可以使用链式操作继续简化代码
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file4() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}

//总体原则：
//在定义一个可能失败的函数时，优先考虑返回Result
//否则就panic!
//可以使用panic的情况：初步项目 后续再完善的情况 例如演示某些概念：unwrap 原型代码：unwrap、expect
//你可以确定Result就是Ok: unwrap
//当代码最终可能处于损坏状态时，最好用panic! 例如：点用你的代码，传入无意义的参数值：panic! 调用外部不可控代码，返回非法状态，你无法修复：panic!
//但如果失败是可预期的：Result
//当你的代码对值进行操作，首先应该验证这些值：panic!
//总结来说：不能通过代码逻辑搞定的用panic!,能搞定的用Result
fn demo() {
    //你可以确定Result就是Ok: unwrap
    let home: IpAddr = "127.0.0.1".parse().unwrap();
}

fn custom_error_validation() {
    loop {
        let guess = "32";
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if guess < 1 || guess > 100 {
            println!("Guess must be between 1 and 100, got {}", guess);
            continue;
        }
    }
}

pub struct Guess {
    value: i32, //默认私有 因为不允许外部修改
}

impl Guess {
    //定义关联函数来自动验证 表示：只要能成功构建实例，就说明这个实例是一定有效的，以后就不用反复验证有效性了
    fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess { value }
    }
    //类似JS getter
    fn value(&self) -> i32 {
        //允许了访问值
        self.value
    }
}

fn custom_error_validation2() {
    loop {
        let guess = "32";
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let guess = Guess::new(guess);
    }
}
