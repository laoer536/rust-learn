//release profile -> dev profile 和 release profile
//dev profile:适用于开发，cargo build
//release profile:适用于发布，cargo build --release

//可以通过发布包来共享你的代码 crates.io

//文档注释：用于生成文档 （使用命令cargo doc）(生成并打开cargo doc --open)
//-生成html文档
//-显示公共API的文档注释：如何使用API
//-使用///
//-支持Markdown
//放置在被说明条目之前

// use publish::kinds::PrimaryColor;
// use publish::utils::mix;
use publish::{PrimaryColor, mix}; //这样简写 在lib.rs中我们已经使用"pub use"将它们提到了顶部
fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
