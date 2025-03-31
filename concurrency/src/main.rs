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

use std::sync::mpsc;
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

    channel_demo3();
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

//使用消息传递的方式来实现安全的并发 Channel
//使用mpsc::channel函数来创建Channel
//mpsc表示 multiple producer, single consumer(多个生产者、一个消费者)
//返回一个tuple(元组)：里面元素分别是发送端、接收端
fn channel_demo() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        //必须获取所有权
        let val = String::from("hi");
        tx.send(val).unwrap(); //send发送消息
        // println!("val is: {}", val); //Error send获取了val的所有权（move了） 不能再使用了
    });
    let received = rx.recv().unwrap(); //recv会一直阻塞当前线程 直到接收到消息（返回Result<T,E>）,如果发送端关闭，就会收到一个错误
    //扩展：另外有try_recv方法：不会阻塞，立即返回Result<T,E>:有数据达到返回Ok,里面包含着数据 否则返回错误 通常会使用循环来检查try_recv的结果
    println!("Got: {}", received);
}

//发送多个值 看到接收端在等待的demo
fn channel_demo2() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });
    for received in rx {
        println!("Got: {}", received); //会每隔200毫秒打印
    }
}

//通过多个克隆创建多个发送者
fn channel_demo3() {
    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("1: hi"),
            String::from("1: from"),
            String::from("1: the"),
            String::from("1: thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap(); //注意这里时克隆出来的发送端
            thread::sleep(Duration::from_millis(200));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });
    for received in rx {
        println!("Got: {}", received); //结果是 tx和tx1发送的内容交替出现
    }
}
