use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数游戏!!!");

    let secret_number = rand::rng().random_range(1..=100);
    // println!("你输入的神秘数字是: {}", secret_number);  //测试用

    loop {
        println!("猜一个数");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                // println!("您输入的不是数字");
                continue;
            }
        };

        println!("猜测的数是: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("你猜小了"),
            Ordering::Greater => println!("你猜大了"),
            Ordering::Equal => {
                println!("你猜对了");
                break;
            }
        }
    }
}
