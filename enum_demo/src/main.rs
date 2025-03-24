enum IpAddrKind {
    V4,
    V6,
}

enum IpAddrKind2 {
    V4(u8, u8, u8, u8), // ()中可以嵌入任何类型
    V6(String),
}

// 也可以为枚举定义方法 和struct一样
impl IpAddrKind2 {
    fn do_something(&self) {}
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let four = route(IpAddrKind::V4);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddrKind2::V4(127, 0, 0, 1); //更简化的表述了更多信息
    let loopback = IpAddrKind2::V6(String::from("::1")); //更简化的表述了更多信息
    home.do_something();

    //Rust中是没有null的 所以引入了Option枚举
    //enum Option<T> {
    //Some(T),
    //None
    //}
    let some_number: Option<i32> = Some(5);
    // let some_number= Some(5); //也可以这样简写 让自动推断
    let some_string: Option<&str> = Some("a string");
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5); //表示可能是类型为i8的5 也可能不存在
    // let sum = x + y; // Error Option<i8> 与 i8 类型不能相加
    let sum = x + y.unwrap_or(0); // Success 使用unwrap_or把y转换为i8
}

fn route(ip_type: IpAddrKind) -> IpAddrKind {
    ip_type
}
