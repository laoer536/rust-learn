//范型可以提高代码复用能力
//范型是具体类型或其他属性的抽象代替
//里面有展位符，编译器在编译时将展位符替换为具体的类型
//展位符命名规范：统一使用大驼峰，简短的话可以使用单个大写字母占位 类似TS
//使用范型的代码和使用具体类型的代码的运行速度是一样的（编译时 单态化）
fn main() {
    println!("Hello, world!");
}

//函数
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

//结构体 struct
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

//针对具体的类型定义方法 impl就不用指定了
impl Point<f32, f32> {
    fn x1(&self) -> f32 {
        self.x
    }
}

fn strut_generic_demo() {
    let p1 = Point { x: 1, y: 2.0 };
    let p2 = Point { x: 2, y: 3.1 };
    let Point { x: "hello", y: "c" };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p1.x(), p2.x());
}

//枚举 enum
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn enum_demo() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
