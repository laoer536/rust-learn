const MY_NAME: &str = "Laoer536";

fn main() {
    println!("Hello, {}!", MY_NAME);

    let x = 2;
    // x=23; // error x默认不可变
    let x = x + 1; // shadow上一个声明"x"
    print!("x is {} \n",x);

    let mut y = 3; // 声明一个可变的y
    y = y + 1;
    print!("y is {} \n",y);

    let z = "    ";
    let z = z.len(); // shadow上一个声明"z"
    print!("z is {} \n",z);

    let mut m = "        123";
    // m = m.len(); // Error usize -> &str 类型不匹配
    m = m.trim();
    println!("m is {} \n",m);
}
