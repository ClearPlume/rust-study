fn main() {
    // 函数调用
    greet_world();
}

fn greet_world() {
    // 变量的创建：其实应该是常量，默认情况下let创建的变量是不可变的
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";

    // 集合的初始化
    let regions = [southern_germany, chinese, english];

    // 通过迭代器完成集合的迭代。在Rust 2021 edition 及以后，迭代器的获取[iter()]可以省略不写
    for region in regions.iter() {
        println!("{}", region);
    }
}
