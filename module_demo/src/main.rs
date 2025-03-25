// src/main.rs是二进制crate模块 二进制crate的入口 生成二进制可执行文件或者exe
use std::collections::HashMap; //std是内部标准库
// use std::fmt;
// use std::io;
// 使用嵌套路径简写 等同于如上
use std::{fmt, io};
//也可以使用as重命名
// use std::collections as StdCollections;

//use std::io;
//use std::io::Write;
//等同于
//use std::io::{self,Write};

//通配符引入
use std::collections::*; //把collections中所有公共条目都引入到作用域 谨慎使用

//使用自定义module
mod some;

//使用外部包
use rand::Rng;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    some::some_fn::test();
}

fn f1() -> fmt::Result {
    Ok(())
}

fn f2() -> io::Result<()> {
    Ok(())
}
