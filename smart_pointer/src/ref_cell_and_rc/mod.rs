use List2::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

//使用Rc<T>和RefCell<T>就可能创造出循环引用，从而发生内存泄漏
//每个项的引用数量不会变成0，值也不会被处理掉。

#[derive(Debug)]
enum List2 {
    Cons(Rc<RefCell<i32>>, Rc<List2>),
    Nil,
}

fn rc_t_rc_demo() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    *value.borrow_mut() += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
