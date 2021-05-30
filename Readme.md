> Date： 2021年5月30日
> Author: Lucy

# Rust 权威指南学习记录

### Chapter01

* done
* rust的安装与helloword

### Chapter02

* done
* cargo的基本用法

### Chapter03

* done
* rust的基本语法

### Chapter04

* done
* rust中关键概念
  * 所有权
  * Copy trait
  * Drop trait
  * Move 语义
  * 切片语义
  * 引用语义

### Chapter05

* done
* rust中使用struct创建自定义模板

### Chapter06

* done
* rust中使用enum创建自定义枚举
  * 枚举可以带有值
  * 枚举变体可以带有不同类型的值
  * Option<T>
  * match 匹配模式穷尽
  * if let 语法糖

### Chapter07

* done
* rust中如何组织管理项目
  * 包：package
    * 单元包：crate
      * 库单元包：lib.rs
      * 二进制单元包
  * 包的访问
  * 访问权限：pub
  * 引入包：use
  * 包的分化

### Chapter08

* done
* rust 中常用集合
  * Vec
    * 可以用枚举变量来简介存储不同类型，但必须能够完全枚举
  * String
    * 有三个视角：字节(bytes)、字符(chars)、字符簇(未内置方法)
    * format!
  * HashMap(存在与std::collections)
    * entry 用于校验key是否存在
  
### Chapter09

* done
* rust 中的错误处理
  * 可恢复错误
    * 返回 Result<T,E>
  * 不可恢复错误
    * 使用 panic! 宏输出
  * ?
  * unwrap
  * expect
  * match 处理错误

### Chapter10

* done
* rust 中引用的生命周期、泛型、trait
  * 引用的生命周期
    * 返回值必须与输入生命周期中的一个匹配
    * 显示声明
    * **生命周期省略规则**

### Chapter11

* done
* rust 中的测试
  * 单元测试
  * 继承测试
