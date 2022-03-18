fn main() {
    ownership();
    mutable();
    part_move();
    reference();
    ref_keyword();
    borrow();
}

/// 所有权
fn ownership() {
    // 使用尽可能多的方法来通过编译
    let x = String::from("hello, world");
    let y = &x; // 借用
    println!("{}, {}", x, *y);
    let x = String::from("hello, world");
    let y = x.clone(); // 克隆
    println!("{}, {}", x, y);
    let x = "hello, world";
    let y = x; // 字符串字面量
    println!("{}, {}", x, y);

    // 函数引用传递
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);
    println!("{}", s2);

    let s = give_ownership();
    println!("{}", s);

    let s = String::from("hello, world");
    print_str(s.clone());
    println!("{}", s);

    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}

fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}

fn give_ownership() -> String {
    String::from("hello, world")
}

fn print_str(s: String) {
    println!("{}", s)
}

/// 引用可变性
fn mutable() {
    let s = String::from("hello, ");
    let mut s1 = s;
    s1.push_str("world");

    let x = Box::new(5);
    let mut y = Box::new(0);
    *y = 4;
    assert_eq!(*x, 5);
}

/// 复合类型的部分移动&引用
fn part_move() {
    let t = (String::from("hello"), String::from("world"));
    let _s = t.0;
    println!("{:?}", t.1);

    let t = (String::from("hello"), String::from("world"));
    let (ref s1, ref s2) = t;
    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}

/// 引用
fn reference() {
    let x = 5;
    let p = &x;
    println!("x 的内存地址是 {:p}", p);

    let x = 5;
    let y = &x;
    assert_eq!(5, *y);

    let mut s = String::from("hello, ");
    borrow_object(&s);

    let ref mut s = String::from("hello, ");
    push_str(s);
    println!("{}", s);

    let mut s = String::from("hello, ");
    let mut p = s;
    p.push_str("world");
    println!("{}", p);
}

fn borrow_object(_s: &String) {}

fn push_str(s: &mut String) {
    s.push_str("world")
}

/// ref 关键字
fn ref_keyword() {
    let c = '中';
    let r1 = &c;
    let ref r2 = c;
    assert_eq!(*r1, *r2);
    assert_eq!(get_addr(r1), get_addr(r2));
}

fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

/// 借用
fn borrow() {
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);

    let s = String::from("hello, ");
    borrow_object_immutable(&s);

    let mut s = String::from("hello, ");
    borrow_object_mut(&s);
    s.push_str("world");

    let mut s = String::from("hello, ");
    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");
    // println!("{}", r1);

    let mut s = String::from("hello, ");
    let _r1 = &mut s;
    let _r2 = &mut s;
    // 在下面增加一行代码人为制造编译错误：cannot borrow `s` as mutable more than once at a time
    // 你不能同时使用 r1 和 r2
    // println!("{}", r1);
}

fn borrow_object_immutable(_s: &String) {}

fn borrow_object_mut(_s: &String) {}
