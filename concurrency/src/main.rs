//并发：
//concurrent:程序的不同部分之间独立的执行
//Parallel:程序的不同部分同时运行
//Rust无畏并发：允许你编写没有细微bug的代码，并在不引入新bug的情况下易于重构
//在大部分OS里，代码运行在进程中，OS同时管理多个进程。
//在你的程序里，各独立部分可以同时运行，运行这些独立部分的就是线程。
//多线程运行:提升性能表现，增加复杂性：无法保障各线程的执行顺序。

//可导致的问题：
//竞争状态，线程以不一致的顺序访问数据资源
//死锁，两个线程彼此等待对方使用完所持有的资源，线程无法继续
//只在某些情况下发生Bug,很难可靠的复制现象和修复

use std::thread;
use std::time::Duration;

//实现线程的方式：
//通过调用的API来创建线程：1:1模型 --- 需要较小的运行时
//语言自己实现的线程（绿色线程）：M:N模型 --- 需要更大的运行时
//Rust: 需要权衡运行时的支持 Rust标准库仅提供1:1模型的线程 （因为有底层功能 所以有第三方包可支持M:N模式）
fn main() {
    //通过thread::spawn创建新的线程
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap(); //阻塞主线程的执行 直到handle所对应的线程结束
    //主线程
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn demo2() {
    let v = vec![1, 2, 3];
    // let handle = thread::spawn(|| {  //Error 因为闭包的生命周期可能比v长
    //     println!("Here's a vector: {:?}", v);
    // });
    //move闭包通常和thread::spawn函数一起使用，它允许你使用其他线程数据 （把一个值的所有权转移到另一个线程）
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}
