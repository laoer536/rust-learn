//集合包含了：Vector String HashMap
//储存在堆内存上面 运行时可变

use std::collections::HashMap; //HashMap不在预导入模块中 所以需要手动导入

fn main() {
    //Vector只能存放相同类型的数据 值在内存中是连续存放的
    // let v :Vec<i32> = Vec::new();
    // let v  = vec![1,2,3,4,5]; // !宏调用 可直接这种方式初始化
    let mut v2 = Vec::new();//能自动推断Vector类型
    v2.push(6);
    demo1();
    demo2();
    traverse_vector_2();
    string_read_demo();
    string_operate_demo();
    hashmap_declare_demo();
}

//索引访问Vector
fn demo1(){
    let v = vec![1,2,3,4,5];
    let third = &v[2];//引用
    println!("The third element is {}", third);
    //使用索引访问
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

//满足同时可变或同时不可变的规则
fn demo2() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; //不可变引用
    // v.push(6); //Error！ 可变引用
    println!("The first element is {}", first);
}

//遍历vector
fn traverse_vector(){
    let v = vec![1,2,3,4,5];
    for i in &v {
        println!("{}", i);
    }
}

//遍历时修改Vector成员
fn traverse_vector_2(){
    let mut v = vec![1,2,3,4,5];
    for i in &mut v {
        *i += 50; //解引用符
    }
    for i in &v {
        println!("{}", i);
    }
}

//使用vector+enum实现存放不同类型的数据
enum SpreadsheetCell{
    Int(i32),
    Float(f64),
    Text(String),
}

fn save_different_type_in_vector(){
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

//字符串：Byte的集合 采用UTF-8编码方式
//核心语言层 &str 字符串切片 不可变
//String类型 来自标准库 运行时可变 String是对Vec<u8>的包装
fn string_declare_demo(){
    // let str1 = String::new(); //无初始值创建
    let data = "initial contents";
    let s = data.to_string(); //可以通过to_string把字符串字面值转化为String类型
    let s1 = "initial contents".to_string(); //可以通过to_string把字符串字面值转化为String类型
    let s2 = String::from("initial contents");//可以通过这种直接创建String类型
}

fn string_operate_demo(){
    let mut s = String::from("foo");
    s.push_str("bar"); //针对字符串切片 不会获得所有权 所以s可以继续使用
    s.push('l'); //针对单个字符(使用单引号) 不会获得所有权 所以s可以继续使用
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s3 = s1 + &s2;//字符串拼接 内部实现类似于 fn(self,&str){...}
    println!("{}", s3);
    // println!("s1:{s1}"); //s1不能再使用 内部实现已经发生了所有权的移动
    println!("s2:{s2}");

    //使用format拼接
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // let s = s1 + "-" + &s2 + "-" + &s3;
    // println!("{}", s);
    // 上面代码可以使用format来更简洁地实现
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
    println!("{}", s1); //不会获得任意一个的所有权 后续都还可以继续使用

    let str = "Hello world";
    let hello = &str[0..5]; //注意需要考虑字符边界 否则会panic
    println!("{}", hello);
}

fn string_read_demo(){
    let s1 = String::from("Hello");
    let s2 = String::from("✨");
    // let h = s1[0]; //Error string indices are ranges of `usize` 不支持索引访问
    let len = s1.len();
    let len2 = s2.len();
    println!("{}, {}", len2, s2); //获取所占子节大小 不同类型的字符不同 这里len2为3

    //rust中字符：子节 标量值
    //标量值
    for c in s1.chars() {
        println!("{}", c);
    }
    //子节
    for b in s2.bytes() {
        println!("{}", b);
    }
    //字形簇（最接近所谓的“字母”）标准库未提供
}

//HashMap<K,V> K键 V值 所有的key必须是同一种类型 值也是 是同构
//默认情况下 使用加密功能强大的Hash函数，可以抵抗拒绝服务Dos攻击。不是最快但最安全的算法。
fn hashmap_declare_demo(){
    // let mut scores:HashMap<String,i32> = HashMap::new();
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect(); //根据键Vec和值Vec使用zip合并成HashMap

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    // map.insert(field_name, field_value); //获取了所有权
    // println!("{field_name}:{field_value}") //Error field_name和field_value已经发生了移动 不能再使用了
    map.insert(&field_name, &field_value); //借用
    println!("{field_name}:{field_value}"); //可以继续使用

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 25);
    let team_name = String::from("Blue");
    //读取值
    let score = scores.get(&team_name);
    match score {
        Some(value) => println!("{}: {}",team_name, value),
        None => println!("{team_name} is None"),
    }
    //遍历
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    //修改
    //替换现有的V
    scores.insert(String::from("Blue"), 25);
    //entry主要主要用来判断某个Key是否存在
    scores.entry(String::from("Blue")).or_insert(50);//如果Blue不存在才插入 这里or_insert不会执行

    //实现统计单词出现的数量
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1; //解引用 修改
    }
    println!("{:?}", map);
}



