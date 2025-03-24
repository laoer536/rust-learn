fn main() {
    let s = String::from("hello world");
    //字符串切片
    // let hello = &s[0..5]; //索引0到4 [0,5) 前闭后开 其实还可以这样写let hello = &s[0..=4]; 两端闭
    let hello = &s[..5]; //简写
    // let world = &s[6..11];//索引6到10 [6,11) 前闭后开 其实还可以这样写let hello = &s[6..=10]; 两端闭
    let world = &s[6..]; // 简写
    // let hello_world = &s[0..];//整个字符切片
    let hello_world = &s[..];//简写
    println!("{} {}", hello, world);
    println!("{}", hello_world);

    //字符串字面值就是切片 另外&str是不可变的
    let s1 = "Hello, world!";
    let word_index = first_word(s1); //直接使用字符串切片
    let s2 = String::from("Hello, world!!!");
    let word_index2 = first_word(&s2[..]); //将String转化为字符串切片再传入引用
    println!("{}", word_index);
    println!("{}", word_index2);

    //扩展数组
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; //数组切片
    println!("{:?}", slice);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
