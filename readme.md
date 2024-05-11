# rust 相关宏
rust相关的宏，大致可以分为两大类，一类是声明式宏、另类是过程宏。
## 声明式宏
使用 macro_rules! 的声明来实现的宏，我们一般称之为声明式宏（Macros by Example）
## 过程宏
过程宏有三种类型
- 自定义 #[derive] 宏在结构体和枚举上指定通过 derive 属性添加的代码。
custom!(...)
- 类属性（Attribute-like）宏定义可用于任意项的自定义属性
#[derive(CustomDerive)]
- 类函数宏看起来像函数不过作用于作为参数传递的token
#[CustomAttribute]


