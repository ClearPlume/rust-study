# 基本类型

## 数值类型

* 整数  
  直接输入的数字字面量，Rust默认它是i32类型
  ```rust
  fn integer() {
    let a: i8 = 100;
    let b: u8 = 200;
    let c: i16 = 32530;
    let d: u16 = 65530;
    let e = 2147483647; // let e: i32 = 2147483647;
    let f: u32 = 4147483620;
  }
  ```

  |  长度   |  有符号类型  |  无符号类型  |
    |:-----:|:-------:|:-------:|
  |  8位   |   i8    |   u8    |
  |  16位  |   i16   |   u16   |
  |  32位  |   i32   |   u32   |
  |  64位  |   i64   |   u64   |
  | 128位  |  i128   |  u128   |
  | 视架构而定 |  isize  |  usize  |

* 浮点数  
  按照长度分为两种，`f32`和`f64`。f64为默认类型
  ```rust
  fn float() {
    let a: f32 = 3.2;
    let b = 5.0; // let b: f64 = 5.0;
  }
  ```

* 数值运算  
  Rust支持所有数值类型的五种基本运算：+ - * / %

  ```rust
  fn calculate() {
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
  }
  ```

* 序列  
  支持创建整数和字符的序列，只有这两种类型可以连续  
  使用`..`创建序列，若要两端包含则用`..=`，如下：

  ```rust
  fn range() {
    for num in 1..5 {
      println!("{}", num); // 1 2 3 4
    }
  
    for num in 1..=5 {
      println!("{}", num); // 1 2 3 4 5
    }

    for char in 'a'..='e' {
      println!("{}", char); // a b c d e
    }
  }
  ```

* 有理数和复数  
  Rust并未直接在标准库中提供这两种数值类型，需引入num库

  ```toml
  # Cargo.toml
  [dependencies]
  num = "0.4.0"
  ```
  num使用如下：
  ```rust
  use num::Complex;

  fn complex() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    println!("{} + {}i", result.re, result.im);
  }
  ```

## 字符、布尔、单元类型

* 字符类型  
  Rust中的字符采用Unicode编码，一个字符占用4个字节
  ```rust
  fn chars() {
    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';
  }
  ```
* 布尔类型  
  Rust中的布尔类型只有两个值：`true` `false`，占用1个字节
  ```rust
  fn boolean() {
    let a = true;
    let b = false;
  }
  ```
* 单元类型  
  Rust提供了一种专用于`函数默认返回值类型`的类型————单元类型。当函数没有明确指定返回值类型时，它的返回值类型就是单元类型。单元类型只有一个值：`()`，不占任何内存。

## 语句、表达式

* 语句  
  执行后没有返回值且**分号结束**的代码行
  ```rust
  fn statement() {
    // 不论有没有返回值，这些代码行都以分号结尾
    let a = 10;
    let b = 5;
    a + b;
    ();
    { // 一个代码块可以视作一个代码行，其最后一个代码行就是它返回的结果
      a * b;
    }
    return ();
  }
  ```
* 表达式  
  有返回值且**没有分号结尾**的代码行
  ```rust
  fn expression() {
    // 这些代码行都有返回值，且都没有分号结尾
    a + b
    no_return()
    number_return()
    { // 一个代码块可以视作一个代码行，其最后一个代码行就是它返回的结果
      let (a, b) = (2, 3);
      a * b
    }
  }

  fn no_return() -> () {}

  fn number_return() -> i32 { 2 }
  ```

## 函数

与其它语言的函数、方法等类似，Rust的函数如下所示：

```rust
fn no_return_implicit() {}

fn no_return_explicit() -> () { () }

fn addition(a: i32, b: i32) -> i32 { a + b }

fn multiply(a: i32, b: i32) -> i32 {
  return a * b;
}
```

其中，包含如下要素：

* 函数定义关键字：`fn`
* 函数名: `no_return_implicit` `no_return_explicit` `addition` `multiply`
* 参数列表: `a: i32, b: i32`
  其中，参数可以有多个，单个参数`a: i32`的结构如下：
    * 参数名: `a` `b`
    * 参数类型: `i32`
* 固定符号: `->`，表示“返回”的含义
* 返回类型: `i32` `()`，描述这个函数将要返回什么类型的数据。若类型是`()`，则可省略不写
* 返回值: ` `&nbsp;`()` `a + b` `a * b`  
  Rust中的函数提供了两种返回值的方式，显式和隐式：
    * 显式  
      与其它语言类似，直接通过`return`语句返回一个值，就像`multiply`函数
        * 函数`multiply`，它的最后一个语句是`return a * b;`，将返回`a * b`的结果
    * 隐式  
      计算函数体中最后一行表达式的值，然后将其返回。这种情况要求函数体最后一行必须是一个表达式，否则函数将返回一个单元类型，也就没有返回值
        * 函数`no_return_implicit`，它的最后没有表达式，将返回一个`()`
        * 函数`no_return_explicit`，它的最后一个表达式是`()`，将返回它计算的结果，也就是`()`
        * 函数`addition`，它的最后一个表达式是`a + b`，将返回该表达式的计算结果
