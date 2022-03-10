# 变量与常量

## 变量

* 变量绑定  
  在Rust中，创建变量其实应该是“对象绑定”。Rust新增了一个“所有权”的概念，创建变量其实是“新建对象并指定它的所有者”，之后都会使用“绑定”描述这一动作
  ```rust
  fn create_variable() {
    // 声明一个变量`a`，创建一个对象`5`，然后将`5`的所有权赋予`a`
    let a = 5;

    // 没有显式指定x的类型，则编译器自动推断
    let x = 5;

    // 显式指定变量类型
    let y: i32 = 5;

    // 直接在变量值后指定类型
    let i = 5i32;

    // 也可在值与类型之间插入下划线以提高可读性
    let j = 5_i32;

    // 若要消除未使用变量在编译时的警告，可在变量名之前添加前置下划线
    let _k = 4;
  }
  ```
* 变量的可变性  
  直接用`let`声明的变量是不可变的，一旦确定了一个值就不可在之后修改
  ```rust
  fn immutable_variable() {
    let x = 5;
    x = 6; // error: cannot assign twice to immutable variable
  }
  ```
  若有一个需要在后续流程中修改的变量，声明变量时用`mut`修饰。`mut`是mutable的缩写
  ```rust
  fn mutable_variable() {
    let mut y = 5;
    println!("The value of y is: {}", y); // The value of y is: 5
    y = 6;
    println!("The value of y is: {}", y); // The value of y is: 6
  }
  ```
* 变量解构  
  Rust有一种数据类型叫“元组”，就是把多个类型组合起来作为一个复合类型存在
  ```rust
  fn tup() {
    let tup: (&str, i32, &str) = ("Tom", 20, "男");
  }
  ```
  变量的解构就是从元组之类的复杂变量里把其中的构成变量匹配出来
  ```rust
  fn deconstruction() {
    let tup: (&str, i32, &str) = ("Tom", 20, "男");
    // 解构时指定变量类型
    let (name, mut age, sex): (&str, i32, &str) = tup;

    // 解构时不指定变量类型，自动推断
    let (name, mut age, sex) = tup;
    println!("name = {}, age = {}, sex = {}", name, age, sex);

    // 解构时可用`_`表示不需要的某个变量，`..`表示忽略之后的全部变量
    let (name, mut age, _) = tup;
    let (name, ..) = tup;

    age = 18;
    assert_eq!(age, 18);
  }
  ```
* 变量遮蔽  
  Rust允许在变量声明时的作用域内再次声明同名变量，后声明的变量将“覆盖”掉先声明的变量
  ```rust
  fn shadowing() {
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
  ```

## 常量

常量与变量的区别：

1. 使用const声明
2. 声明时必须指定常量类型
3. 不可在同一作用域内重复声明
   ```rust
   fn constant() {
     // 常量必须在定义时就写明类型
     const A: i32 = 9;

     // 常量不可在同一作用域内重复定义
     const A: i32 = 8; // A value named `A` has already been defined in this block

     {
         // 只要不在同一作用域，即可使用一样的名称
         const A: i32 = 90;
     }
   }
   ```
