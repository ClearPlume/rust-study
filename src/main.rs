fn main() {
    define_variable();
    mut_variable();
    deconstruction();
    constant();
}

/// 变量的定义
fn define_variable() {
    // 创建一个数值对象“5”，将其所有者指定为x
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

    // Rust允许在同一作用域内使用相同名称声明新的变量
    let chars = "abcdefghi";
    println!("{}", chars); // abcdefghi

    // 此处并非为chars绑定另一个对象，而是声明了一个新的变量，只是它的名字凑巧也叫“chars”
    // 后声明的chars将把之前的chars遮蔽掉
    let chars = 9;
    println!("{}", chars); // 9

    {// inner
        // 此处是重新声明了一个名为chars的变量
        let chars = chars + 1;
        println!("{}", chars); // 10
    }
    // 这里chars的值不会受到inner作用域内部操作的影响
    println!("{}", chars); // 9
}

/// 变量的可变性
fn mut_variable() {
    let x = 5;
    println!("The value of x is: {}", x);

    // 直接用let绑定的变量不可再更改值
    // x = 6; // error: cannot assign twice to immutable variable
    // println!("The value of x is: {}", x);

    // 如需在初次绑定之后修改变量值，则在变量绑定时用mut修饰
    // mut是mutable的缩写
    let mut y = 5;
    println!("The value of y is: {}", y);

    // 创建对象“6”，绑定给变量y
    y = 6;
    println!("The value of y is: {}", y);
}

/// 变量的解构
fn deconstruction() {
    // (true, false)是一个元组，将其解构为两个变量
    // let (a, mut b): (bool, bool) = (true, false);
    // (true, false)是一个元组，将其解构为两个变量，变量类型自动推断
    let (a, mut b) = (true, false);
    println!("a = {}, b = {}", a, b);

    b = true;
    assert_eq!(a, b);
}

/// 常量
fn constant() {
    // 常量必须在定义时就写明类型
    const A: i32 = 9;

    // 常量不可在同一作用域内重复定义
    // const A: i32 = 8; // A value named `A` has already been defined in this block

    {
        // 只要不在同一作用域，即可使用一样的名称
        const A: i32 = 90;
    }
}
