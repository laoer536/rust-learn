use std::io::{Read, Write};
use std::{fs, thread};
//构建多线程web服务器
//在socket上监听TCP连接
//解析少量的HTTP请求
//创建一个合适的HTTP响应
//使用线程池改进服务器的吞吐量
use rust_web_server::ThreadPool;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //单线程方式
    // for stream in listener.incoming() {
    //     //单个stream就是一个服务器和客户端的连接
    //     let stream = stream.unwrap();
    //     handle_client(stream);
    // }

    //多线程
    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();
    //     thread::spawn(|| handle_client(stream)); //来一个请求就会创建一个线程 服务器可能会崩掉
    // }

    //多线程最终版本
    let pool = ThreadPool::new(4);
    for stream in listener.incoming().take(2) {
        //take 2 表示只能处理两次请求
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_client(stream);
        })
    }
    println!("Shutting down all workers."); //主线程打印 这里会比其他线程更快
}

fn handle_client(mut stream: TcpStream) {
    //stream内部状态可能会改变 所以声明为mut
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap(); //把数据放到缓存区里面
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5)); //休眠5秒
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(file_name).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
