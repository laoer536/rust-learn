fn main() {
    //不可变引用
    let s1 = String::from("hello");
    let len = calculate_length(&s1); //&s1 表示传递的是s1的引用 &符号表示：引用某些值而不取得其所有权
    println!("The length of '{}' is {}.", s1, len); //因为s1传递的是引用所以s1不会发生move 所以这里s1仍然是可用的

    //可变引用
    let mut s2 = String::from("hello");
    let len2 = calculate_length_mut(&mut s2);
    println!("The length of '{}' is {}.", s2, len2);

    //防止多指针（数据竞争）
    let mut s3 = String::from("hello");
    //在特定作用域内，对某一块数据只能有一个可变引用（编译时防止数据竞争）
    //发生数据竞争的情况：1、两个或多个指针访问同一个数据 2、至少有一个指针用于写入数据 3、没有使用任何机制来同步对数据的访问
    let s4 = &mut s3;
    let s5 = &mut s3; // Error

    //可以通过创建新的作用域来允许非同时的创建多个可变引用
    let mut s6 = String::from("hello");
    {
        let s7 = &mut s6;
    }
    let s8 = &mut s6; //不同的作用域 可以创建

    //或者非同时也可
    let mut s9 = String::from("hello");
    let s10 = &mut s9;
    // 使用 s10 完成操作后，再创建 s11
    // let s11 = &mut s9; // 此时 s4 已不再使用，可以创建 s5

    //不可以同时拥有一个可变引用和一个不可变引用 另外多个不变的引用是可以的
    let mut m = String::from("hello");
    let n1 = &m;
    let n2 = &m;
    let m1 = &mut m; // Error
    println!("{} {} {}", n1, n2, m1);

    //悬空指针
    // let r1 = dangle();
}

fn calculate_length(s: &String) -> usize { //把引用作为函数参数的行为叫做借用
    // s.push_str(" hello"); //Error 不可以修改借用的值 引用默认也是不可变的
    s.len()  //这里 s指向s1 (指针)
}

fn calculate_length_mut(s: &mut String) -> usize { //把引用作为函数参数的行为叫做借用
    s.push_str(" hello"); //Success 可以修改可变引用的值
    s.len()  //这里 s指向s2 (指针)
}

// fn dangle() -> &String { //&String 报错Error
//     let s = String::from("hello");
//     &s //s离开作用域时会被销毁 所以这里&s是一个悬空指针 rust会在编译阶段报错 让你避免这种情况的发生
// }