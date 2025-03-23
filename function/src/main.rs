fn main() {
    println!("Hello, rust!");
    print_x_y(5, 10);
    print_z();
    println!("get_plus_five(): {}", get_plus_five(6)); // 5 + 6
    compare_five(6)
}

fn print_x_y(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn print_z() {
    let base = 3;
    let z = {
        //声明的值可以是表达式
        let base = 4;
        // base + 2; // 加了分号返回的是元组 ->（）
        base + 2 // -> i32 默认返回最后一个表达式
    };
    let condition = true;
    let m = if condition { 5 } else { 6 }; //声明的值可以是表达式 -> 5
    // let m = if condition { 5 } else { "6" }; // Error 类型必须是确定的 不存在即是数字又是字符的情况
    println!("The value of base is: {}", base); // -> 3 而不是4 类似JS 作用域
    println!("The value of z is: {}", z);
    println!("The value of m is: {}", m);
}

fn get_plus_five(x: i32) -> i32 {
    5 + x // 默认返回最后一个表达式
}

// 控制流
fn compare_five(number: i32) {
    let five = 5;
    if number > five {
        // 判断条件的类型必须是布尔类型 不存在隐式转换 这和JavaScript不同
        println!("The number {} is bigger than five", number);
    } else if number == five {
        // 不等于使用 !=
        println!("The number {} is equal to five", number);
    } else {
        println!("The number {} is smaller than five", number);
    }
}
