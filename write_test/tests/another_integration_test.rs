//tests目录下的就是集成测试 cargo会自动识别 并且每一个tests下面的文件 都会创建一个单独的crate 这些文件不共享行为(与src下的文件规则不同) 所以需要将被测试库导入
use write_test::{self, add};

//无需标注#[cfg(test)],tests目录被特殊对待
//运行指定的集成测试：1、cargo test 函数名 2、运行某个测试文件内的所有测试：cargo test --test 文件名
#[test]
fn it_really_adds_two() {
    assert_eq!(5, add(2, 4));
}
