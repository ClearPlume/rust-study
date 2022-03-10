fn main() {
    variable_define();
    variable_mutable();
    variable_effect_area();
    variable_shadowing();
    unused_variable();
    deconstruction();
    deconstruction_define();
}

/// 变量绑定
fn variable_define() {
    let x: i32 = 5; // 未初始化，但被使用
    let _y: i32; // 未初始化，也未被使用。用前缀下划线可消除编译警告
    println!("{} is equal to 5", x);
}

/// 变量的可变性
fn variable_mutable() {
    let mut x = 1;
    x += 2;
    println!("{} = 3", x);
}

/// 变量作用域
fn variable_effect_area() {
    let x: i32 = 10;
    {
        let y: i32 = 5;
        println!("x 的值是 {}, y 的值是 {}", x, y);
    }
    println!("x 的值是 {}", x);

    let x = "hello";
    println!("{}, world", x);
}

/// 变量遮蔽
fn variable_shadowing() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }
    assert_eq!(x, 5);
    let x = 42;
    println!("{}", x); // 输出 "42".

    let mut x: i32 = 1;
    x = 7;
    // 遮蔽且再次绑定
    let _x = x;
    let _y = 4;
    // 遮蔽
    let _y = "I can also be bound to text!";
}

/// 未使用的变量
fn unused_variable() {
    // 使用变量
    let x = 1;
    println!("{}", x);

    // 使用前缀下划线
    let _y = 9;
}

/// 变量解构
fn deconstruction() {
    let (mut x, y) = (1, 2);
    x += 2;
    assert_eq!(x, 3);
    assert_eq!(y, 2);
}

/// 解构式赋值
fn deconstruction_define() {
    let (x, y);
    (x, _) = (3, 4);
    [.., y] = [1, 2];
    assert_eq!([x, y], [3, 2]);
    println!("x = {}, y = {}", x, y)
}
