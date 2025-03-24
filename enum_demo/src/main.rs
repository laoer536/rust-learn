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
}

fn route(ip_type: IpAddrKind) -> IpAddrKind {
    ip_type
}
