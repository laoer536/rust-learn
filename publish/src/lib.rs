//! # Publish Crate
//!
//! `my_crate` is a collection of utilities to make performing certain calculations more convenient.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = publish::add_one(arg);
///
/// assert_eq!(6,answer);
/// ```

//常用章节：
//# Examples
//Panics: 函数可能发生panic的场景
//Errors: 如果返回Result,描述可能的错误种类，以及可导致错误的条件
//Safety: 如果函数处于unsafe调用，就应该解释函数unsafe的原因，以及调用者保持使用前提。

//运行cargo test会把文档注释中的示例代码作为测试来运行

//为包含注释的项添加文档注释 符号：//!
//这类注释通常描述crate和模块
// crate root(按惯例src/lib.rs) 或者一个模块内，将crate或模块作为一个整体进行记录

//使用 pub use 导出方便使用的公共API
//目的：my_crate::some_module::anther_module::UsefulType; -> my_crate::UsefulType; //让其导入更加简洁方便

//发布包需要在crates.io上获取login token
//使用cargo publish时默认git需要先提交 可以通过加参数 --allow-dirty 来不用先提交也可以发布
//crate一旦发布，就是永久性的并且该版本无法覆盖，代码无法删除，目的是为了保证依赖于该版本的项目可以继续正常工作
//使用cargo yank 可以从Crates.io撤回版本：防止新项目依赖于该版本，已经存在的项目可以继续将其作为依赖并可下载（因为已发布的是不能再删除的）命令：cargo yank --vers 0.1.0 表示撤回0.1.0版本 取消撤回cargo yank --vers 0.1.0 --undo
pub fn add_one(x: i32) -> i32 {
    x + 1
}
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The second colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Orange
    }
}
