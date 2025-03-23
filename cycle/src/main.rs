fn main() {
    // ======================
    // 1. loop 循环（无条件无限循环）
    // ======================
    let mut count = 0;

    loop {
        println!("Loop循环: count = {}", count);
        count += 1;

        // 手动设置退出条件
        if count >= 3 {
            break; // 必须显式break退出 否则将无限循环
        }
    }
    /* 输出：
    Loop循环: count = 0
    Loop循环: count = 1
    Loop循环: count = 2
    */

    // ======================
    // 2. while 循环（条件循环）
    // ======================
    let mut temp = 5;

    while temp > 0 { // 不满足时退出 同样如果初始情况不满足 则不会进入循环
        println!("While倒计时: {}", temp);
        temp -= 1;
    }
    println!("发射！");
    /* 输出：
    While倒计时: 5
    While倒计时: 4
    While倒计时: 3
    While倒计时: 2
    While倒计时: 1
    发射！
    */

    // ======================
    // 3. for 循环（迭代循环）
    // ======================
    // 1. 基本范围表达式（左闭右开）
    println!("=== 1. 基本范围 1..5 ===");
    for i in 1..5 { // 生成 1-4 的序列
        println!("值: {}", i);
    }
    /* 输出：
    值: 1
    值: 2
    值: 3
    值: 4
    */

    // 2. 包含上限的范围表达式
    println!("\n=== 2. 闭合范围 1..=5 ===");
    for i in 1..=5 { // 生成 1-5 的序列
        println!("值: {}", i);
    }
    /* 输出：
    值: 1
    值: 2
    值: 3
    值: 4
    值: 5
    */

    // 3. 直接遍历数组元素
    let fruits = ["🍇", "🍒", "🍉"];

    for fruit in fruits.iter() {
        println!("水果: {}", fruit);
    }
    /* 输出：
    For水果: 苹果
    For水果: 香蕉
    For水果: 橙子
    */


    // 4. 使用 iter() 方法遍历
    println!("\n=== 4. 使用 iter() 方法 ===");
    for num in fruits.iter() { // 显式创建迭代器
        println!("迭代元素: {}", num);
    }
    /* 输出同3 */

    // 5. 带索引的遍历
    println!("\n=== 5. 带索引的遍历 ===");
    for (index, &value) in fruits.iter().enumerate() {
        println!("索引: {}, 值: {}", index, value);
    }
    /* 输出：
    索引: 0, 值: 10
    索引: 1, 值: 20
    索引: 2, 值: 30
    索引: 3, 值: 40
    */

    // 6. 反向遍历
    println!("\n=== 6. 反向遍历 ===");
    for i in (1..5).rev() { // 反转迭代器
        println!("倒序: {}", i);
    }
    /* 输出：
    倒序: 4
    倒序: 3
    倒序: 2
    倒序: 1
    */
}