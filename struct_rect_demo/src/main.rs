#[derive(Debug)] //为了debug Rectangle 信息
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("area:{:#?}", area(&rect));
    println!("rect = {:#?}", rect); //后续能继续使用是因为借用
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
