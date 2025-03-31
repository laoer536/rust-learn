//Rc<T>: 启用多重所有权的引用计数类型（引用计数，类似JS中的引用计数） 有时，一个值会有多个所有者，为了支持多重所有权 只能用于单线程场景
//仅允许不可变引用
mod ref_cell;
mod ref_cell_and_rc;
mod weak_demo;

use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn rc_t_demo() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a)); //1
    // a.clone(); //这样也可以 但会带来性能损耗
    let b = Cons(3, Rc::clone(&a)); //推荐 Rc::clone
    println!("count after creating b = {}", Rc::strong_count(&a)); //2
    {
        let c = Cons(4, Rc::clone(&a)); //推荐 Rc::clone
        println!("count after creating c = {}", Rc::strong_count(&a)); //3
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a)); //2 因为c那里离开了作用域 引用计数减1
}
