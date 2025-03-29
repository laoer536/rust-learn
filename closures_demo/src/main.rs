use std::thread;
use std::time::Duration;

//闭包：可以捕获其所在环境的匿名函数
//是匿名函数
//保存为变量、作为参数
//可以在一个地方创建闭包，然后在另一个上下文中调用闭包来完成运算
//可从其定义的作用域捕获值
//闭包不要求标注参数和返回值类型，闭包很短小，只在狭小的上下文中工作，编译器通常能推断出类型
fn main() {
    println!("Hello, world!");
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2)); //模拟耗时算法
    intensity
}

//创建一个struct,它持有闭包及其调用结果：只会在需要结果时才执行闭包，可以缓存结果->这个模式通常叫做记忆化或延迟计算
//所有的闭包都至少实现了以下Trait之一：fn,FnMut,FnOnce
//FnOnce 取得所有权
//FnMut 可变借用
//Fn 不可变借用
//创建闭包时，通过闭包对环境值的引用，Rust推断出具体使用那个Trait
//所有闭包都实现了FnOnce
//没有移动捕获变量的实现了FnMut
//无需可变访问捕获变量的闭包实现了Fn
struct Cacher<T>
where
    T: Fn(u32) -> u32, //当指定Fn Trait bound之一时，首先用Fn,基于闭包体里面的情况，如果需要FnOnce或FnMut，编译器会再告诉你
{
    calculation: T,
    value: Option<u32>,
}
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Self {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        //缓存
        //匿名函数 这里也可以手动标注类型 |num:u32|->u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        // println!("Today, do {} pushups!", simulated_expensive_calculation(intensity)); 每次运行都会耗时
        // println!("Next, do {} situps", simulated_expensive_calculation(intensity)); 每次运行都会耗时
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps", expensive_closure.value(intensity));
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!(
            "Today, run for {} minutes!",
            expensive_closure.value(intensity)
        );
    }
}

fn example_closure_fn() {
    let expensive_closure = |num| num;
    let s = expensive_closure(String::from("hello"));
    // let n = expensive_closure(3);//因为第一次调用推断出来的已经是String类型了 再次调用传入u32类型就不对了 也就是说在闭包没有手动标注类型的情况下 第一次调用就决定了他的入参和出参类型 后续不会再改变
}

//闭包可以访问定义它的作用域的变量，而普通函数不能
fn demo() {
    let x = 4;
    let equal_to_x = |z| z == x; //可以捕获 但会产生额外的内存开销
    // fn equal_to_x(z: i32) -> bool {
    //     z == x //Error 普通函数不能访问定义它的作用域的变量
    // };
    let y = 4;
    assert!(equal_to_x(y));
}

fn demo2() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    // println!("{:?}", x); // Error x的所有权已经移动到了闭包里面 不能再使用了
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}
