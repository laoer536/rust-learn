use std::slice;

//高级特性：
//不安全Rust
//高级Trait
//高级类型
//高级函数和闭包
//宏
fn main() {
    println!("Hello, world!");
    //匹配命名变量
    //隐藏着第二个语言，他没有强制性内存安全保证： Unsafe Rust (不安全的Rust)
    //和普通的Rust一样，但提供了额外的“超能力”
    //Unsafe Rust存在的原因：静态分析是保守的（使用Unsafe Rust: 我知道自己在做什么，并承担相应风险。另外计算机本身就是不安全的，Rust需要能够进行底层系统编程）。

    //超能力：
    //使用unsafe关键字来切换到unsafe Rust, 开启一个块，里面放着unsafe代码
    //Unsafe Rust里可执行的四个动作(超能力):
    //解引用原始指针
    //调用unsafe函数或者方法
    //访问或修改可变的静态变量
    //实现unsafe trait

    //注意：
    //unsafe并没有关闭借用检查或停用其他安全检查
    //任何内存安全相关的错误必须留在unsafe块里
    //尽可能隔离unsafe代码，最好将其封装在安全的抽象里，提供安全的API

    //解引用原始指针：可变的：*mut T, 不可变的：*const T。意味着指针在解引用后不能直接对其进行赋值 (注意上述*不是解引用符号，他是类型名的一部分)
    //与指针不同原始指针：
    // 允许通过同时具有不可变和可变指针或多个指向同一位置的可变指针来忽略借用规则。
    // 无法保证能指向合理的内存。
    //允许为null
    //不实现任何自动清理
    //放弃保证的安全，换取更好的性能/与其他语言或硬件接口的能力

    let mut num = 5;
    let r1 = &num as *const i32; //使用原始指针的原因：1、与C语言进行接口 2、构建借用检查器无法理解的安全对象
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
    let address = 0x012345usize;
    let r = address as *const i32;
    unsafe {
        println!("r is: {}", *r);
    }

    //unsafe函数或方法：在定义前面加上了unsafe关键字
    //调用前需手动满足一些条件（主要看文档），因为Rust无法对这些条件进行验证
    //需要在unsafe块中调用
    unsafe { dangerous() }

    //创建unsafe代码的安全抽象
    //函数包含unsafe代码并不意味着需要将整个函数标记为unsafe
    //将unsafe代码包裹在安全函数中是一个常见的抽象
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    //split_at_mut的实现
    // fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    //     let len = slice.len();
    //     let ptr = slice.as_mut_ptr(); //返回一个原始指针 *mut i32
    //     assert!(mid <= len);
    //     使用了原始指针 可以局部抽象为unsafe 无需标记整个函数为unsafe
    //     unsafe {
    //         (
    //             slice::from_raw_parts_mut(ptr, mid),
    //             slice::from_raw_parts_mut(ptr.add(mid), len - mid),
    //         )
    //     }
    // }
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    //使用extern函数调用外部代码
    //extern关键字：简化创建和使用外部函数接口（FFI）的过程。
    //外部函数接口（FFI，Foreign Function Interface）: 它允许一种编程语言定义函数，并让其他编程语言能够调用这些函数
    //应用二进制接口（ABI，Application Binary Interface）: 定义函数在汇编层的调用方式
    //“C” ABI是最常见的ABI，它遵循C语言的ABI

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    //访问或修改一个可变静态变量
    //Rust支持全局变量，但因为所有权机制可能产生某些问题，例如数据竞争
    //在Rust中，全局变量叫做静态（static）变量
    static HELLO_WORLD: &str = "Hello, world!";
    println!("{}", HELLO_WORLD);

    //静态变量与常量类似
    //全大写的蛇形命名 必须标注类型 静态变量只能储存'static 生命周期的引用，无需显式标注
    //访问不可变的静态变量是安全的
    //不同点：
    //静态变量：有固定的内存地址，使用它的值总会访问同样的数据，可以是改变的，访问和修改静态可变变量是不安全（unsafe）的 常量：允许使用它们的时候对数据进行复制
    static mut COUNTER: u32 = 0;
    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

unsafe fn dangerous() {}

unsafe extern "C" {
    fn abs(input: i32) -> i32; //extern里面的内容都是unsafe的 只能通过unsafe块调用
}

//从其他语言调用Rust函数
//可以使用extern创建接口，其他语言通过它们可以调用Rust的函数
//在fn前添加extern关键字，并指定ABI
//还需添加#[no_mangle]注解：避免Rust在编译时改变它的名称
#[no_mangle]
pub extern "C" fn call_from_c() {
    //这个函数就可以让C语言调用
    println!("Just called a Rust function from C!");
}

//实现不安全(unsafe)的trait
//当某个trait中存在至少一个方法拥有编译器无法校验的不安全因素时，就称这个trait是不安全的。
//声明unsafe trait: 在定义的前面加unsafe关键字 该trait只能在unsafe代码块中实现
unsafe trait Foo {}

unsafe impl Foo for i32 {}

//何时使用unsafe代码
//编译无法保证内存安全，保证unsafe代码正确并不简单
//有充足的理由使用unsafe代码时，就可以这样做
//通过显式标记unsafe, 可以在出现问题时轻松定位
