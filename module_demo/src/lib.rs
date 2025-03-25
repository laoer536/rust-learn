// src/lib.rs是库crate模块 是库“crate”的入口 代表库crate 用来提供可复用的代码库 建议所有mod在这里汇总 方便其他位置用绝对路径使用
// 这里的 “crate”指向的是src/lib.rs内容
// use crate::serve_order;

mod front_of_house;
mod some; //这里mod声明后 some为库crate的子模块 如果子模块要使用 use crate::xxx导入 那么这里的crate代表就是src/lib.rs的内容 为库crate的根
// 在src下面找名字为front_of_house的rs文件

fn serve_order() {}
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); //super表示上一级
        //当然也可以用绝对路径
        // crate::serve_order();
    }
    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,      //公共
        seasonal_fruit: String, //私有
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        //枚举不太一样 若枚举是公共的 那么成员也就自动公共了 不需要再使用pub声明
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant_2() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from("blueberries"); // Error seasonal_fruit是私有字段
}

//将hosting提取到当前作用域 这里提到了顶级
use crate::front_of_house::hosting; //本身以及如果有成员的话仍然遵循私有性原则 这里是私有的 仅在该作用域中可以用 可以前面加pub使其变为公有
pub fn eat_at_restaurant_3() {
    //这里hosting可以直接使用
    hosting::add_to_waitlist();
}
