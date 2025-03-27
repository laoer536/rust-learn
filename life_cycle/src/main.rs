use std::fmt::Display;

//Rust中每个引用都有自己的生命周期
//生命周期：引用保持有效的作用域
//大多数情况：生命周期是隐式的、可被推断的
//当引用的生命周期可能以不同的方式互相关联时：手动标注生命周期。
//生命周期的主要目标：避免悬垂引用
fn main() {
    println!("Hello, world!");
    {
        let r;
        {
            let x = 1;
            r = &x; //Error r使用了x的引用 但出了作用域 x就被销毁了 r也就无效了 被引用的生命周期比r短 所以这里就报错了
        }
        println!("r: {}", r);
    }
}

fn demo() {
    let x = 5;
    let r = &x; //这里x的生命周期比r长 即引用的生命周期是大于等于的情况 这里就没有问题 这里x是一直有效的
    println!("r: {}", r);
} //函数执行完毕后x才会被销毁（栈内存回收）所以x的生命周期包含了整个函数

fn demo2() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    let result2 = longest2(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

//使用'a 标注生命周期 都是'a 表示生命周期都是一样的 这样就没问题
//生命周期的标注描述了多个引用的生命周期间的关系，但不影响生命周期
//书写方式 '+全小写 结合引用就是“&'x 类型”或者“&'x mut 类型” 前者不可变引用 后者是可变引用 单个生命周期描述没有任何意义
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //这里其实最终的生命周期区的是x,y中生命周期比较短的那一个（如果x,y生命周期不一样的话）
    if x.len() > y.len() { x } else { y }
}

//悬空指针
// fn longest2<'a>(x:&'a str,y:&'a str) ->&'a str {
//     let result = String::from("abc");
//     result.as_str() //Error 悬空指针 这里执行完之后 result已经被销毁了
// }

//解决上述问题 我们不返回引用 将所有权给调用者即可 总结一下：在某些情况下，要想不被清理，那么就给出所有权即可
fn longest2<'a>(x: &'a str, y: &'a str) -> String {
    let result = String::from("abc");
    result
}

struct ImportantExcerpt<'a> {
    part: &'a str, //part的引用的生命周期必须比ImportantExcerpt实例长才行 因为这里实例存在 那么part会一直保持这个引用
}

fn demo3() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

//生命周期省略规则（满足规则的情况下无需手动标注，编译器会自动标注）：
//在Rust引用分析中所编入的模式称为生命周期省略规则。他们是一些特殊情况，由编译器考虑，无需显式标注生命周期
//生命周期省略规则不会提供完整的推断，如果应用规则后，引用的生命周期仍然模糊不清，那么就会编译错误，这个时候就需要手动标注了，注明引用间的相互关系

//每个引用类型的参数都有自己的生命周期
//如果只有一个输入生命周期参数，那么该生命周期被赋予给所有的输出生命周期参数
//如果有多个输入生命周期参数，但其中一个是&self或&mut self(是方法)，那么self的生命周期会被赋予给所有的输出生命周期参数

//在函数/方法的参数处 ：输入生命周期
//在函数/方法的返回值处：输出生命周期

//这里补充一下方法和函数的概念
//
//方法是面向实例的操作，依赖 self 参数和类型上下文。
//
// 函数是独立的逻辑单元，无隐式上下文绑定。
//
// 关联函数是特殊的函数，与类型关联但无需实例调用（如构造函数）。

// ### 关键区别总结
// | 特性                | 方法（Method）                          | 函数（Function）               |
// |---------------------|----------------------------------------|-------------------------------|
// | 定义位置            | 在 `impl` 块中                          | 任意位置（模块、函数体等）     |
// | 第一个参数          | 必须有 `self`                           | 无强制要求                    |
// | 调用方式            | `instance.method()`                    | `function()` 或 `module::function()` |
// | 访问私有字段        | 可直接访问（同模块）                   | 需在模块内或通过公共接口       |
// | 典型用途            | 操作实例的上下文                       | 独立逻辑或工具函数             |

//这里举个例子 假设我们不手动标注 那么编译器来自动标注的话就变成了 &str就会报错 因为不确定返回类型的生命周期
// fn longest_default<'a,'b>(x: &'a str, y: &'b str) -> &str {
//     //这里其实最终的生命周期区的是x,y中生命周期比较短的那一个（如果x,y生命周期不一样的话）
//     if x.len() > y.len() { x } else { y }
// }

//特殊的生命周期标注 'static , 表示在整个程序的持续时间内 生命周期都是有效的 谨慎使用
// let s:&'static str = "abcd";

//泛型参数类型、Trait Bound、生命周期同时使用的demo
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() { x } else { y }
}
