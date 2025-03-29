pub fn add_tow(x: i32) -> i32 {
    x + 2
}

// 运行cargo test 那么所有项目的测试都会run 可通过cargo test -p add-one来指定运行特定项目的测试用例
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = add_tow(3);
        assert_eq!(result, 5);
    }
}
