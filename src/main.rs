use num::Complex;

fn main() {}

/// 数值类型
fn numeric() {
    // 整数分为有符号(i)和无符号(u)两种，类型定义的形式为：有无符号 + 类型大小
    // 默认情况下，直接输入的数字会被当做i32处理
    let a: i8 = 100;
    let b: u8 = 200;
    let c: i16 = 32530;
    let d: u16 = 65530;
    let e = 2147483647; // let e: i32 = 2147483647;
    let f: u32 = 4147483620;
    let g: i64 = 120256558845548465;
    let h: u64 = 520256558845548465;

    // 浮点数按照长度分为两种，f32和f64
    // 直接输入的浮点数默认为f64类型
    let i: f32 = 6.2831852;
    let j = 6.2831852547852694; // let j: f64 = 6.2831852547852694;

    // Rust支持所有数值类型的五种基本运算：+ - * / %
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
}

/// 序列
fn range() {
    // Rust针对整数和字符类型提供了序列，使用“..”创建序列
    for i in 1..5 {
        println!("{}", i); // 1 2 3 4
    }

    for i in 1..=5 {
        println!("{}", i); // 1 2 3 4 5
    }

    for c in 'a'..='e' {
        println!("{}", c); // a b c d e
    }
}

/// 有理数和复数
fn complex() {
    // Rust并未直接在标准库中提供这两种数值类型，需引入num库
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    println!("{} + {}i", result.re, result.im);
}

/// 字符类型
fn chars() {
    // Rust中的字符采用Unicode编码，一个字符占用4个字节
    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';
}

/// 布尔类型
fn boolean() {
    // Rust中的布尔类型只有两个值：true false，占用1个字节
    let a = true;
    let b = false;
}

/// 单元类型
fn unit() {
    // Rust提供了一种专用于`函数默认返回值类型`的类型————单元类型。当函数没有明确指定返回值类型时，它的返回值类型就是单元类型。单元类型只有一个值：`()`，不占任何内存。
    let unit = (); // 创建单元类型的字面量
    let bool = boolean(); // 调用boolean方法，接收其返回值
}

/// 语句和表达式
fn statement_expression() -> i32 {
    // Rust把执行后没有返回值且分号结束的代码行叫做“语句”，有返回值且没有分号结尾的代码行叫做“表达式”
    let a = 5; // 语句，它没有返回值

    {
        let a = 5;
        a + 1
    } // 这个代码块返回了表达式“a + 1”的结果，是一条表达式

    {
        let a = 9;
    } // 这个代码块没有返回值，是一条语句

    1 + 1 // 表达式，它不是分号结束，同时会返回一个值。同时它作为一个函数的结束，同时作为这个函数的“返回语句”
    // return 1 + 1; // 语句，尽管它的作用是返回一个值，但它本身没有返回值且以分号结尾
}

/// 函数
fn function() {
    // 函数的基本定义：
    // fn function_name(param1: i32, param2: u32, .., paramN: &str) -> i32
}
