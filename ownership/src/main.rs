fn main() {
    //将值传递给函数将发生移动或者复制
    let s = String::from("hello world");
    take_ownership(s); // s move到函数take_ownership中
    // println!("s:{}", s); // Error s已经被移动了 s不能在之后使用
    let x = 5;
    makes_copy(x); // x是i32类型，实现了copy trait 所以传递的是x的副本 而不是发生了移动
    println!("x is {}", x); // x 能够在之后使用

    //函数在返回值的过程中同样也会发生所有权的转移
    let s1 = gives_ownership(); // gives_ownership返回值some_string移动给了s1
    let s2 = String::from("hello");
    let s3 = take_and_gives_back(s2); //move: s2 -> a_string -> s3
    //之后离开作用域  s3销毁 s2已经移动了 s1被销毁
}

fn take_ownership(some_string: String) {
    //some_string是s move后的值
    println!("{}", some_string);
    // 这里调用drop 释放some_string
}

fn makes_copy(some_integer: i32) {
    //some_integer是x的副本
    println!("{}", some_integer);
    // 这里释放some_integer 因为是副本 所以不会对x造成影响
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn take_and_gives_back(a_string: String) -> String {
    a_string
}
