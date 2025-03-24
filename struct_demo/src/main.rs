//一旦struct的实例是可变的，那么实例中所有的字段都是可变的
//没有引用存放的话：只要struct实例是有效的，那么里面的字段数据也是有效的
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

//struct里也可以存放引用，但这需要使用生命周期
//有引用存放的话：生命周期保证只要struct实例是有效的，那么里面的引用也是有效的。
// struct User2 {
//     username: &str, //Error
//     email: &str, //Error
//     sign_in_count: u64,
//     active: bool,
// }

fn main() {
    println!("Hello, world!");
    let mut user1 = User {
        //没有mut声明成员是不能修改的
        email: String::from("coderme@qq.com"),
        username: String::from("Laoer536"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("abcd123@email.com");
    let user2 = build_user(String::from("coderme@qq.com"), String::from("Laoer536"));
    let user3 = User {
        email: String::from("345@qq.com"),
        username: String::from("Laoer666"),
        ..user1 //合并 类似JS ES6的扩展运算符 但是有一点不同的是显式指定的字段优先级高于 .. 语法中的继承字段 所以显式声明的字段不会被覆盖掉
                //这里会move user1 有所有权问题 可以使用..user1.clone()创建副本解决
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        // email:email,
        // username:username,
        //简写 类似JS ES6 同名简写
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn tuple_struct_demo() {
    struct Color(i32, i32, i32); //tuple struct
    struct Point(i32, i32, i32); //tuple struct
    // black和origin是不同的类型，是不同的tuple struct实例
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

//Unit-Like Struct（没有任何字段）//适用于需要在某个类型上实现某个trait,但是在里面又没有想要储存的数据
