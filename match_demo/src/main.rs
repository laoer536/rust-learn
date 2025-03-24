//控制流运算符
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn main() {
    let c = Coin::Quarter(UsState::Alaska);
    println!("{}", value_in_cents(c));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let v = 0u8;
    match v {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), //使用“_”来处理剩余的情况 类似switch的default
    }

    //有的时候我们只需要处理match中的一种情况而而忽略其他的情况
    let v2 = Some(0u8);
    // if let 模式匹配
    if let Some(3) = v2 {
        println!("three");
    }
    //以上代码相当于
    match v2 {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = v2 {
        println!("three");
    } else {
        println!("others");
    }
    //以上代码相当于
    match v2 {
        Some(3) => println!("three"),
        _ => println!("others"),
    }

    //何时使用 if let？
    // 需要同时解构数据（如从 Option/Result 中取值）。
    //
    // 需要匹配范围、绑定变量或复杂模式时。
    //
    // 不推荐仅用于简单值的相等性检查（优先用 ==）。
}

fn value_in_cents(coin: Coin) -> u8 {
    //类似switch
    match coin {
        Coin::Penny => {
            //多个语句用花括号
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:#?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        //每种模式都需要考虑到 不能删除 但可以用“_”来处理剩余情况
        None => None,
        Some(i) => Some(i + 1),
    }
}
