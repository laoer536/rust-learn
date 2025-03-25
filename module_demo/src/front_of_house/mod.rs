pub mod hosting {
    pub fn add_to_waitlist() {}
    fn seat_at_table() {}
}

mod serving {
    fn take_order() {}
    fn serve_order() {}
    fn take_payment() {}
}

pub fn eat_at_restaurant() {
    //根是隐式crate模块 rust中crate代表 src/lib.rs 称为库crate  ::相当于下一级
    crate::front_of_house::hosting::add_to_waitlist(); //绝对路径调用 后续代码变更不会一起移动的情况下 一般采用这种 因为你不好确定未来是否会不会一直都是一起移动的
    //这里front_of_house可用是因为都在同一个根级 所以front_of_house不用声明pub也能使用
    hosting::add_to_waitlist() //相对路径调用 后续代码变更会一起移动的情况下
}
