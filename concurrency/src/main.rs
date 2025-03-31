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

use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

//实现线程的方式：
//通过调用的API来创建线程：1:1模型 --- 需要较小的运行时
//语言自己实现的线程（绿色线程）：M:N模型 --- 需要更大的运行时
//Rust: 需要权衡运行时的支持 Rust标准库仅提供1:1模型的线程 （因为有底层功能 所以有第三方包可支持M:N模式）

//Go语言的名言：不要用共享内存来通信，要用通信来共享内存。(Rust的Channel)
//但Rust还支持通过共享状态来实现并发。Channel类似单所有权：一旦将值的所有权移至Channel,就无法使用它了。
//共享内存并发类似多所有权：多个线程可以同时访问同一块内存。

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

//使用Mutex来每次只允许一个线程来访问数据
//Mutex是mutual exclusion(互斥锁)的简写
//在同一时刻，Mutex只允许一个线程来访问某些数据
//想要访问数据：线程必须先获取互斥锁（lock:lock数据结构是mutex的一部分，它能跟踪谁对数据拥有独占访问权） mutex通常被描述为：通过锁定系统来保护它所持有的数据

//Mutex的两条规则：在使用数据之前，必须尝试获取锁（lock）。使用完mutex所保护的数据，必须对数据进行解锁，以便其他线程可以获取锁。

fn mutex_demo() {
    let m = Mutex::new(5); //Mutex<T>也是一个智能指针
    {
        let mut num = m.lock().unwrap(); //lock方法会阻塞当前线程，知道获取到这个锁
        *num = 6;
    }
    println!("m = {:?}", m);
}

fn mutex_demo2() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    //创建了10个线程
    // for _ in 0..10 {
    //     let handle = thread::spawn(move || { //Error 第二次循环的时候会报错已经被move了 因为counter的所有权已经交给第一次循环了
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }

    //使用Arc<T>来进行原子引用计数 A:表示atomic，原子的 ，它可以用于并发场景 Arc<T>和Rc<T>的API是相同的 但Arc<T>需要性能作为代价
    //Mutex<T>提供了内部可变性，和Cell家族一样 但有死锁风险
    //我们使用RefCell<T>来改变Rc<T>里面的内容
    //我们使用Mutex<T>来改变Arc<T>里面的内容
    // 创建了10个线程
    for _ in 0..10 {
        let counter = Arc::clone(&counter); //这样就可以了
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}

//死锁

//1. 互斥锁（Mutex<T>）的嵌套锁定
// 当多个线程以不同顺序获取多个锁时，可能形成循环依赖：
fn deadlock() {
    let lock1 = Arc::new(Mutex::new(0));
    let lock2 = Arc::new(Mutex::new(0));

    let l1 = Arc::clone(&lock1);
    let l2 = Arc::clone(&lock2);
    let t1 = thread::spawn(move || {
        let _a = l1.lock().unwrap(); // 先锁 lock1
        let _b = l2.lock().unwrap(); // 再锁 lock2
    });

    let l1 = Arc::clone(&lock1);
    let l2 = Arc::clone(&lock2);
    let t2 = thread::spawn(move || {
        let _b = l2.lock().unwrap(); // 先锁 lock2
        let _a = l1.lock().unwrap(); // 再锁 lock1
    });

    t1.join().unwrap();
    t2.join().unwrap(); // 可能死锁！
}

//2. 单线程中的重复锁定
// 同一线程多次尝试锁定同一个 Mutex（如递归调用中未释放锁）：
fn deadlock1() {
    let lock = Mutex::new(0);
    let _a = lock.lock().unwrap(); // 第一次锁定
    let _b = lock.lock().unwrap(); // 再次尝试锁定（阻塞）
    // 死锁：第二次锁定会阻塞，直到第一次锁释放，但永远无法执行到这里
}

//3. 通道（channel）的阻塞等待
// 发送端和接收端因未正确处理消息而互相等待：
fn deadlock2() {
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    let tx1_clone = mpsc::Sender::clone(&tx1);
    thread::spawn(move || {
        let msg = rx2.recv().unwrap(); // 等待接收消息
        tx1_clone.send(msg).unwrap();
    });

    thread::spawn(move || {
        let msg = rx1.recv().unwrap(); // 等待接收消息
        tx2.send(msg).unwrap();
    });

    // 两个线程都在等待对方先发送消息，导致死锁
}
