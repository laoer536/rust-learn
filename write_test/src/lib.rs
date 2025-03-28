//控制cargo test的行为：添加命令参数
//默认行为：并行运行（多个线程）所有测试 捕获（不显示）所有错误，使读取与测试结果相关的输出更容易。（因为是并行，所以需要确保测试之间不会相互依赖。不依赖于某个共享状态（环境、工作目录、环境变量等））
//命令行参数：1、针对cargo test的参数：紧跟cargo test之后 2、针对测试可执行程序：放在 -- 之后
// cargo test -- --test-threads=1 表示使用一个线程来运行测试 更慢但出现干扰的情况就比较少
//默认情况下测试通过的情况下控制台不会输出用户打印的内容 例如println!打印的内容 但可以加命令行参数解决 cargo test -- --show-output
//默认情况下会运行所有测试 但可以通过指定测试名来运行单个测试 例如cargo test result_test_demo或者批量指定 只需指定测试名的一部分（模块名也可以）例如cargo test it(会测试it_works_1和it_works_2)

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub struct Rectangle {
    width: u64,
    height: u64,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        };
        Guess { value }
    }
    pub fn new2(value: u32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

//测试函数panic就表示失败
//每个测试运行在一个新线程，当主线程看见某个测试线程挂掉了（发生了panic），那个测试标记为失败。

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        let result = add(2, 2);
        assert_eq!(result, 4); //测试相等 相当于== 断言失败时会自动打印出两个参数的值
    }

    #[test]
    fn it_works_2() {
        let result = add(2, 2);
        assert_ne!(result, 4); //测试不等 相当于!= 断言失败时会自动打印出两个参数的值
    }

    #[test]
    #[ignore] //可忽略该测试 可通过 cargo test -- --ignored 来运行被ignored的测试
    fn it_works_3() {
        let result = add(2, 2);
        assert_eq!(result, 4); //测试不等 相当于!= 断言失败时会自动打印出两个参数的值
    }

    #[test]
    fn another_function() {
        panic!("Make this test fail"); //panic 导致这个测试失败
    }

    //使用assert!宏检查测试结果
    //来自标准库，用来确定某个状态是否为true
    //true:测试通过
    //false:调用panic!,测试失败
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 7,
            height: 8,
        };
        let smaller = Rectangle {
            width: 1,
            height: 5,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        ); //自定义错误信息 assert的第二个参数
        //扩展 assert_ne!和assert_eq!自定义在第三个参数传入即可
    }

    //验证是否如预期发生了错误
    //验证代码在特定情况下是否发生了panic
    //should_panic属性
    //函数panic:测试通过
    //函数没有panic:测试失败
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
    //让should_panic更精确
    //添加可选的expected参数
    //将检查失败消息中是否包含所指定的文字
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100, got 200.")]
    fn greater_than_100_should_panic() {
        Guess::new2(200);
    }

    //在测试中使用Result<T,E>
    //无需panic,可使用Result<T,E>作为返回类型编写测试：
    //返回Ok:测试通过
    //返回Err:测试失败
    //注意：不要在使用Result<T,E>编写的测试上标注#[should_panic] 因为这种情况下失败会返回Err 不会发生恐慌
    #[test]
    fn result_test_demo() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("tow plus two does not equal four"))
        }
    }
}
