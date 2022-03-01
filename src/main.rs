fn main() {
    define_variable();
}

/// 变量的定义
fn define_variable() {
    // 创建一个数值对象“5”，将其所有者指定为“x”
    // 没有显式指定x的类型，则编译器自动推断
    let x = 5;

    // 显式指定变量类型
    let y: i32 = 5;

    // 直接在变量值后指定类型
    let i = 5i32;

    // 也可在值与类型之间插入下划线以提高可读性
    let j = 5_i32;

    let sum = (x + y) + (i + j);

    // “println!” 并非函数调用，它是一个宏
    // 将格式化字符串输出到标准控制台
    // “{}”是一个占位符，会自动替换为sum的值
    println!("(x + y) + (i + j) = {}", sum);
}
