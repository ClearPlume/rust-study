# 变量与常量
## 变量
在Rust中，变量的“定义”其实应该是“绑定”。  

之后都会使用“绑定”描述这一动作，因为Rust新增了一个“所有权”的概念，创建变量其实是“新建对象并指定它的所有者”。

本节包含如下内容：
1. 变量绑定
2. 变量可变性
3. 变量解构
4. 变量遮蔽

## 常量
与变量的区别：
1. 不可在同一作用域内重复声明
2. 使用const声明
