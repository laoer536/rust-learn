//这里的的crate指向库crate 即src/lib.rs 因为some已经声明为其子模块
use crate::hosting::add_to_waitlist;
pub fn test() {
    println!("hello world");
    add_to_waitlist();
}
