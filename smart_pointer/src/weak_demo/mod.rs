use std::cell::RefCell;
use std::rc::{Rc, Weak};

// 定义树节点结构体
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>, // 父节点：Weak 避免循环引用，RefCell 提供内部可变性
    children: RefCell<Vec<Rc<Node>>>, // 子节点列表：Rc 共享所有权，RefCell 允许动态修改
}

fn main() {
    // 创建叶子节点（无父节点，无子节点）
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()), // 初始化为空的 Weak 引用
        children: RefCell::new(vec![]),    // 空子节点列表
    });

    // 打印初始引用计数（强引用=1，弱引用=0）
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    // 内部作用域定义分支节点
    {
        // 创建分支节点（包含叶子节点作为子节点）
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()), // 分支节点暂时无父节点
            children: RefCell::new(vec![Rc::clone(&leaf)]), // 克隆 leaf 的 Rc，强引用+1
        });

        // 将 leaf 的父节点指向 branch（Weak 引用）
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // branch 的弱引用+1

        // 打印分支节点的引用计数（强引用=1，弱引用=1）
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );

        // 打印叶子节点的引用计数（强引用=2，弱引用=0）
        // 强引用=2：leaf 自身 + branch.children 中的克隆
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    } // branch 离开作用域，强引用-1（变为0），branch 被释放

    // 尝试访问 leaf 的父节点（已随 branch 被释放，返回 None）
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // 打印最终的引用计数（强引用=1，弱引用=0）
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}
