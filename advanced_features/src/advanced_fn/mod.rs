//函数指针
//可以将函数传递给其他函数 函数在传递的过程中会被强制转换成fn类型（函数指针）

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
}

//函数指针和闭包的不同
//fn是一个类型，不是一个trait：可以直接指定fn为参数类型，不用声明一个以Fn trait为约束的泛型参数

//函数指针实现了全部三种闭包trait(Fn,FnMut,FnOnce)
//总是可以把函数指针用作参数传递给一个接收闭包的函数
//所以，倾向于搭配闭包trait的泛型来编写函数：可以同时接收闭包和普通函数

//某些情景，只想接收fn而不是闭包：
//与外部不支持闭包的代码交互：C函数

fn main2() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect(); //传闭包

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect(); //传函数

    enum Status {
        Value(u32),
        Stop,
    }
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect(); //传函数

    //返回闭包
    //闭包使用trait进行表达，无法在函数中直接返回一个闭包，可以将一个实现了该trait的具体类型作为返回值
    // fn returns_closure() -> Fn(i32) -> i32 {
    //     //Error Rust无法知道返回的闭包需要多少空间
    //     |x| x + 1
    // }

    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
}
