pub fn add_one(x: i32) -> i32 {
    x + 1
}

// 运行cargo test 那么所有项目的测试都会run 可通过cargo test -p add-one来指定运行特定项目的测试用例
#[cfg(test)]
mod tests {
    use super::add_one;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
