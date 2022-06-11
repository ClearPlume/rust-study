# 编辑器属性标记

使用`#[]`括起来，使用的时候大致和方法/函数调用差不多。有如下一些：

```rust
#[allow(unused_variables)] // 允许未使用的变量，不要抛出警告
#[allow(unused_mut)] // 允许未使用的mut变量，不要抛出警告
#[allow(unused)] // 允许任意未使用的变量、mut、导入等，不要抛出警告
#[allow(dead_code)] // 允许永不返回的函数
#[test] // 表示这个方法是一个测试方法，相当于Java中的@Test注解
fn just_make_compile_pass() {}
```

官方文档：[Attributes - The Rust Reference](https://doc.rust-lang.org/reference/attributes.html)

# 复合类型

由多个类型组合而成的类型，如：枚举，结构体，元组等

```rust
// 枚举
enum Season {
    SPRING,
    SUMMER,
    AUTUMN,
    WINTER
}

// 结构体
struct User {
    id: String,
    name: String,
    active: bool
}

// 元组
#[allow(unused_variables)]
fn tuple() {
    let tup = (10, "Tom", false);
    let tom = User {
        id: String::from("4B068EB5-0941-4645-1E98-FC077D530A61"),
        name: String::from("Tom"),
        active: true
    };
}
```

## 字符串与切片

下例中的字符串字面量`name`不是字符串类型，它的确切类型是“字符串切片的引用”(&str)。若要创建字符串类型的变量，可使用String结构体中的`from`方法或者调用字符串字面量的`to_string`方法

```rust
#[allow(unused_variables)]
fn string_snippet() {
    let gender = "Male"; // &str
    let name = String::from("Tom"); // String
    let sign = "This is sign.".to_string(); // String
}
```

### 切片

切片是一个引用，它是对集合中一段连续序列的引用。对于字符串而言，切片就是子串，如下所示：

```rust
#[allow(unused_variables)]
fn slice() {
    let hello_world = "Hello, world!".to_string(); // String

    let hello = &hello_world[0..5]; // &str
    let world = &hello_world[7..12]; // &str
}
```

`hello`和`world`只是引用了`hello_world`中的一部分内容，以这种方式指定引用内容的边界：`[0..5]`

#### 创建切片的语法

这种语法名叫“range 序列”，是使用方括号包括的一个序列: `[开始索引..终止索引]`，其中开始索引是切片中第一个元素的索引位置，而终止索引是最后一个元素后面的索引位置，是一个`右半开区间`。在切片数据结构内部会保存开始的位置和切片的长度，其中长度是通过`终止索引 - 开始索引`的方式计算得来的。

range序列的一些语法特性：

* 序列从索引0开始时，0可以忽略不写
* 序列以集合(字符串)最后一个元素的索引为终止索引时，终止索引可忽略不写
* 若要完整的集合序列，可不写开始索引和终止索引

```rust
#[allow(unused_variables)]
fn range() {
    let nums = [1, 2, 3, 4, 5];

    let nums = &nums[..3];
    let nums = &nums[2..];
    let nums = &nums[..];
}
```

### 字符串和字符串字面量

Rust 中的字符串类型是`String`结构体，是一系列字符组成的连续集合，是一个可增长、可改变，具有所有权的 UTF-8 字符串

字符串字面量是String的切片引用类型`str`，字面量最终会被硬编码进可执行文件，str则指向可执行文件中字面量的固定位置，因此字面量是不可变的

#### 字符串操作

* 创建字符串  
  `let str = String::new();`
* 字符串新增字符串内容  
  `str.push_str("Hello, world");`
* 字符串新增字符内容  
  `str.push('!'); // Hello, world!`
* &str与字符串的转换

```rust
#[allow(unused_variables)]
fn convert() {
    let string = "Hello, world".to_string();
    let string = String::from("Hello, world");
    let str = &string[..]; // 若以这种方式新建引用，则要注意类似“你好”这种一个字符不是一个字节的情况。如果开始索引或终止索引落在一个字符的内部，则会导致报错：byte index 2 is not a char boundary; it is inside '你' (bytes 0..3) of `你好`
    let str = string.as_str();
}
```

* 字符串连接

```rust
#[allow(unused_variables)]
fn concat() {
    let hello = String.from("Hello, ");
    let hello = String.from("world!");
    let hello_world = hello + &world;
}
```

## 元组

把多个类型组合起来，就是元组。元组的长度是固定的，其中的元素类型也是固定的

### 创建元组

用括号将多个值组合起来，就是元组。它的类型是`(每个值的类型)`

```rust
#[allow(unused_variables)]
fn tup_create() {
    let tom = (1, "Tom", true);// tom: (i32, &str, bool)
}
```

### 访问元组中元素

* 以点的形式，按照元素下标访问

```rust
#[allow(unused_variables)]
fn tup_access_index() {
    let tom_id = tom.0;
    let tom_name = tom.1;
    let tom_active = tom.2;
}
```

* 通过模式匹配，直接访问元组中的所有元素

```rust
#[allow(unused_variables)]
fn tup_access_pattern() {
    let (tom_id, tom_name, tom_active) = tom;
}
```

### 在方法中使用元组，一次返回多个值

```rust
#[allow(unused_variables)]
fn tup_ret_val() {
    let name = "Tom".to_string();
    let (name, name_len) = str_len(name);// "Tom", 3
}

fn str_len(str: String) -> (String, i32) {
    return (str, str.len());
}
```

## 结构体

结构体的定义与元组类似，都是多个类型的组合。不过结构体多了一点特性，它可以为结构体内每一个元素取名，不再需要按照定义顺序访问结构体字段

### 结构体语法

#### 定义结构体

结构体的构成：

- 关键字`struct`
- 结构体`名称`
- 一对大括号`{}`，以及大括号内部的`几个有名字的元素(字段/变量)`

如下所示，为一个用户结构体：

```rust
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u32
}
```

上述结构体名为`User`，定义了四个字段：`username, email, active, sign_in_count`

#### 创建结构体实例

```rust
fn define_user() {
    let tom = User {
        email: String::from("example@test.com"),
        username: String::from("Tom"),
        active: true,
        sign_in_count: 2
    };

    // 以下语句只有在tom为可变变量时，才可编译通过
    // tom.active = false;
}
```

创建结构体实例时，有两点需要注意：

1. 每个字段都要初始化
2. 初始化字段的顺序不需要与字段定义的顺序一致
3. 必须要把结构体实例声明为可变的，才能修改结构体字段值。Rust 不支持把单个字段标记为可变

#### 简化结构体创建

若创建结构体实例时使用了变量，且变量名和结构体字段名一致，则可简写：

```rust
fn create_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 0
    }
}
```

#### 结构体更新语法

根据已有结构体实例创建新的结构体实例时，若新结构体相对于旧结构体包含了值相同的字段，则可简写：

```rust
fn update_user(tom: User) -> User {
    User {
        username: String::from("Tim"),
        sign_in_count: 0,
        ..tom // ".."表示除了明确写出来的字段，其余字段值直接从tom中获取
    }
}
```

与赋值操作`=`类似，结构体更新也涉及到所有权的转移。以上述结构体转移为例，`username`和`email`是String类型，没有实现`Copy`特征，因此结构体更新时它们的所有权会被转移到新结构体内，tom结构体中只有`active`和`sign_in_count`可使用

#### 结构体的内存排列

依旧以User结构体为例：

<table style='text-align: center'>
    <tr>
        <td>字段名</td>
        <td>username</td>
        <td>email</td>
        <td>active</td>
        <td>sign_in_count</td>
    </tr>
    <tr>
        <td>字段类型</td>
        <td>String</td>
        <td>String</td>
        <td>bool</td>
        <td>u32</td>
    </tr>
    <tr>
        <td>内存表示</td>
        <td>ptr/size/capacity</td>
        <td>ptr/size/capacity</td>
        <td>value</td>
        <td>value</td>
    </tr>
</table>

#### 元组结构体

结构体必须有名称，但结构体的字段可以没有名称，如下：

```rust
struct Color(u32, u32, u32);

struct Point(i32, i32);

struct Point3D(i32, i32, i32);

#[allow(unused_variables)]
fn tuple_struct() {
    let white = Color(255, 255, 255);
    let coordinate = Point(-1, -1);
    let coordinate3d = Point3D(1, 1, 1);
}
```

#### 单元结构体

若结构体没有任何字段，则为单元结构体。这种结构体作为“内容不重要，行为是重点”的类型存在，如下：

```rust
struct AlwaysEquals;

// AlwaysEquals 的内容无所谓，它的行为是我关心的，所以为它实现特征
impl SomeTrait for AlwaysEquals {}

// AlwaysEquals 的内容无所谓，它的行为是我关心的，所以为它定义方法
impl AlwaysEquals {
    fn print() -> &str {
        "永远相等"
    }
}
```

#### 打印结构体信息

如果需要将结构体的字段及其值打印输出，需要为其实现`Display/Debug`特征，或使用`#[derive(Debug)]`派生实现：

```rust
#[derive(Debug)]
struct AlwaysEquals;

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "User {{ username: {}, email: {}, active: {}, sign_in_count: {} }}",
            self.username, self.email, self.active, self.sign_in_count
        )
    }
}

impl Debug for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "User {{ username: {}, email: {}, active: {}, sign_in_count: {} }}",
            self.username, self.email, self.active, self.sign_in_count
        )
    }
}
```

实现`Display`特征后，可以直接使用`{}`在格式字符中输出：`println!("Tom: {}", tom)`  
实现`Debug`特征后，可以使用`{:?}`在格式字符中输出：`println!("Tom: {:?}", tom)`。若要更好的输出格式，可以把`{:?}`换成`{:#?}`

使用`dbg!`可以输出包括代码文件名、行号、表达式、表达式的值在内的所有信息：

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn write_with_dbg() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
```

输出结果如下：

<pre>
$ cargo run
[src/main.rs:10] 30 * scale = 60
[src/main.rs:14] &rect1 = Rectangle {
    width: 60,
    height: 50,
}</pre>
