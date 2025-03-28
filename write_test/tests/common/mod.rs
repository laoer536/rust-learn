//如果不单独提成一个mod 那么cargo test就会当作测试文件来运行 只是没有测试用例 （原：tests/common.rs）

//这样之后 我们就可以把这个模块当作辅助模块 可以在其他测试用例中使用
pub fn setup() {}

//扩展：针对binary crate的集成测试（二进制crate）
//如果项目是binary crate(只含有src/main.rs，没有src/lib.rs)
//那么不能在tests目录下创建集成测试
//无法把main.rs的函数导入作用域，因为只有library crate才能暴露函数给其他crate用，binary crate意味着独立运行
