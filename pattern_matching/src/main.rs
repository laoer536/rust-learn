//模式是Rust中的一种特殊语法，用于匹配复杂和简单类型的结构
//将模式与匹配表达式和其他构造结合使用，可以更好地控制程序的控制流
//模式由以下元素组成：字面值、解构的数组、enum\struct\tuple 变量 通配符占位符
//想要使用模式，需要将其与某个值进行比较：如果模式匹配，就可以在代码中使用这个值的相应部分

fn main() {
    println!("Hello, world!");

    //match的Arm 详尽（需要写出所有可能性 可以使用“_”来匹配除已指定的情况之外的其他所有情况）
    // match VAlUE {
    //     PATTERN => EXPRESSION,
    //     PATTERN => EXPRESSION,
    //     PATTERN => EXPRESSION,
    //     PATTERN => EXPRESSION,
    // }

    //if let表达式主要是作为一种简短的方式来等价的代替只有一个匹配项的match
    //if let可选else或else if let 另外不会检查穷尽性
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    //while let 条件循环
    //只要模式继续满足匹配的条件，那就允许while循环一直运行

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top); //打印顺序 3 2 1
    }

    //for 循环
    let v = vec![1, 2, 3];
    for (index, value) in v.iter().enumerate() {
        //(index, value) 就是模式
        println!("{} is at index {}", value, index);
    }

    //let语句 let PATTERN = EXPRESSION
    let a = 5;
    let (x, y, z) = (1, 2, 3);

    let point = (3, 5);
    print_coordinates(&point)
}

//函数参数 参数参数也可以是模式
fn foo(x: i32) {
    //
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    //相当于ES6的解构
    //
    println!("Current location: ({}, {})", x, y);
}

//模式的两种形式：可辩驳的、无可辩驳的
//能匹配任何可能传递的值的模式：无可辩驳的 例如let x = 5;
//对某些可能的值，无法进行匹配的模式：可辩驳的 例如if let Some(x) = a_value
//函数参数、let语句、for循环只接受无可辩驳的模式
//if let 和 while let 接受可辩驳和无可辩驳的模式
fn demo() {
    let a: Option<i32> = Some(5);
    // let Some(x) = a; // Error pattern `None` not covered 没有考虑None的情况
    if let Some(x) = a {
        println!("Got value: {}", x);
    } // 解决办法
}

//模式匹配语法
fn demo2() {
    //直接匹配字面值
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    //匹配命名变量
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    //多重模式 “或” --->> '|'
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    //使用 ..= 来匹配某个范围的值
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    //解构以分解值
    //可以使用模式来解构struct、enum、tuple，从而引用这些类型值的不同部分
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    // let Point { x: a, y: b } = p; //卧槽 解构终于出来的 可用于结构体的解构 并重命名解构声明 类似ES6
    // assert_eq!(0, a);
    // assert_eq!(7, b);
    //简写
    let Point { x, y } = p; //卧槽 解构终于出来的 可用于结构体的解构 类似ES6
    assert_eq!(0, x);
    assert_eq!(7, y);
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    //解构枚举
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => println!("The Quit variant has no data to quit."),
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }

    //解构嵌套的struct和enum
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    enum Message2 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }
    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h, s, v
            )
        }
        _ => (),
    }

    //解构结构体和元组(struct和tuple)
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 }); //妙

    //在模式匹配中忽略值
    fn foo(_: i32, y: i32) {
        // _ 忽略整个值
        println!("This code only uses the y parameter: {}", y);
        // println!(": {}", _); //Error _是被忽略的值 无效
    }
    fn run() {
        foo(3, 4);
    }
    run();

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            // _ 匹配其他值
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            // 第二个和第四个我们用不上 这里叫忽略某一部分
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }

    // let _x = 5; //可以理解为临时的 暂时用不到 否则会报没有使用的警告
    // let y = 10; //Warning 没有使用的警告
    // let s = Some(String::from("hello"));
    //使用以_开头的名称
    // if let Some(_s) = s {
    //     println!("found a string");
    // }
    // println!("{:?}", s); //Error 不能使用s了 因为s已经移动到了_s
    let s = Some(String::from("hello"));
    if let Some(_) = s {
        //_会保证不会发生所有权的移动
        println!("found a string");
    }
    println!("{:?}", s); //没有错误

    //.. 忽略值的剩余部分
    struct Point2 {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point2 { x: 0, y: 0, z: 0 };
    match origin {
        Point2 { x, .. } => println!("x is {}", x),
    }
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
    // match numbers {
    //     (.., second, ..) => { // Error second的取值会有歧义 不行
    //         println!("Some numbers: {}", second);
    //     }
    // }

    //使用match守卫来提供额外条件
    //match守卫就是match arm模式后额外的if条件，想要匹配该条件也必须满足
    //match守卫适用于比单独模式更复杂的场景
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x), //if x < 5 这段代码就是match的守卫
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {:?}", n), //if n == y 这段代码就是match的守卫
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"), //多重模式匹配
        _ => println!("no"),
    }

    //@绑定 @符号让我们可以创建一个变量，该变量可以在测试某个值是否与模式匹配的同时保存该值

    enum Message3 {
        Hello { id: i32 },
    }
    let msg2 = Message3::Hello { id: 5 };
    match msg2 {
        Message3::Hello {
            id: id_variable @ 3..=7, //使用@ 将匹配的值存到id_variable中
        } => {
            println!("Found an id in range: {}", id_variable)
        }

        Message3::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message3::Hello { id } => println!("Found some other id: {}", id),
    }
}
