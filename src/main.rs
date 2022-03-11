extern crate core;

use std::mem::size_of_val;
use std::ops::{Range, RangeInclusive};

fn main() {
    integer();
    float();
    range();
    computing();
    char();
    boolean();
    unit();
    statement_expression();
    function();
    diverging_function();
}

/// 整数
fn integer() {
    let x: i32 = 5;
    let mut y = 5;
    println!("y = {}", y);
    y = x;
    println!("y = {}", y);

    let _z = 10; // 这里 z 的类型是: i32

    let _v: u16 = 38_u8 as u16; // 类型转换: as

    let a = 5;
    assert_eq!("i32".to_string(), type_of(&a));

    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    let v1 = 247_u8 + 8_u8;
    let v2 = i8::checked_add(117, 8).unwrap();
    println!("v1, v2: {}, {}", v1, v2);

    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert_eq!(v, 1597);
}

/// 获取传入参数的类型，返回类型的字符串形式，例如  "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String { format!("{}", std::any::type_name::<T>()) }

/// 浮点数
fn float() {
    let _x = 1_000.000_1; // f64
    let _y: f32 = 0.12; // f32
    let _z = 0.01_f64; // f64

    // assert!(0.1 + 0.2 == 0.3);
    assert_eq!(0.1 + 0.2, 0.3_f32);
    assert_eq!((0.1 + 0.2) as f32, 0.3);
}

/// 序列
fn range() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }
    assert_eq!(sum, -5);

    for c in 'a'..='z' {
        println!("{}", c as i32);
    }

    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
}

fn computing() {
    assert_eq!(1 + 2, 3);

    assert_eq!(1_i32 - 2, -1);
    assert_eq!(1_i8 - 2, -1 as i8);

    assert_eq!(3 * 50, 150);

    assert_eq!(9.6 / 3.2, 3.0 as f32);

    assert_eq!(24 % 5, 4);

    assert!(true && false == false);
    assert!(true || false == true);
    assert_eq!(!true, false);

    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

/// 字符
fn char() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4);
    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4);
    println!("Success!");

    let c1 = '中';
    print_char(c1);
}

fn print_char(c: char) { println!("{}", c) }

/// 布尔
//noinspection ALL
fn boolean() {
    let _f: bool = false;
    let t = true;
    if t {
        println!("Success!")
    }

    let f = true;
    let t = true || false;
    assert_eq!(t, f);
    println!("Success!")
}

/// 单元类型
fn unit() {
    let v = ();
    assert_eq!(v, implicitly_ret_unit());
    println!("Success!");

    let unit: () = ();
    assert_eq!(size_of_val(&unit), 0);
    println!("Success!")
}

fn implicitly_ret_unit() {
    println!("I will return a ()")
}

/// 语句，表达式
fn statement_expression() {
    let v = {
        let mut x = 1;
        x = x + 2;
        x
    };
    assert_eq!(v, 3);
    let v = {
        let mut x = 1;
        x += 2
    };
    assert_eq!(v, ());

    let s = add(1, 2);
    assert_eq!(s, 3);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn function() {
    let (x, y) = (1, 2);
    let s = sum(x, y);
    assert_eq!(s, 3);

    print();

    // never_return();

    let b = false;
    let _v = match b {
        true => 1,
        // 发散函数也可以用于 `match` 表达式，用于替代任何类型的值
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic")
        }
    };
    println!("Excercise Failed if printing out this line!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn print() -> () {
    println!("hello,world");
}

// fn never_return() -> ! {
//     panic!("This function never return")
// }

fn diverging_function() {
    println!("Success!");
    get_option(5_u8);
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {}
        _ => {}
    };
    never_return_fn()
}

fn never_return_fn() -> ! {
    // unimplemented!()
    loop {}
    panic!()
}
