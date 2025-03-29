// use rand;
use add_one;

//安装二进制crate(binary crate)(可运行程序): cargo install （安装在用户/.cargo/bin目录下)
fn main() {
    let num = 10;
    println!("{} plus one is {}", num, add_one::add_one(num));
}
