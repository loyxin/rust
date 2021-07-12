use std::fmt; // 导入 `fmt` 模块。

// 定义一个包含单个 `Vec` 的结构体 `List`。
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 使用元组的下标获取值，并创建一个 `vec` 的引用。
        let vec = &self.0;

        write!(f, "[")?;

        // 使用 `v` 对 `vec` 进行迭代，并用 `count` 记录迭代次数。
        for (count, v) in vec.iter().enumerate() {
            // 对每个元素（第一个元素除外）加上逗号。
            // 使用 `?` 或 `try!` 来返回错误。
            write!(f, "{0} : {1}", count, v)?;
            if count != vec.len()-1 {write!(f, ", ")?;}
            // if count%2 == 0 { write!(f, " : ")?; }
            // else {write!(f, " , ")?;}
            // write!(f, "{}", v)?;

        }

        // 加上配对中括号，并返回一个 fmt::Result 值。
        write!(f, "]")
    }
}

// 不能实现 第三方 trait 第三方 类型
// 孤儿规则
// compile panic
impl<T> fmt::Display for Vec<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 使用元组的下标获取值，并创建一个 `vec` 的引用。

        write!(f, "[")?;

        // 使用 `v` 对 `vec` 进行迭代，并用 `count` 记录迭代次数。
        for (count, v) in self.iter().enumerate() {
            // 对每个元素（第一个元素除外）加上逗号。
            // 使用 `?` 或 `try!` 来返回错误。
            write!(f, "{0} : {1}", count, v)?;
            if count != self.len()-1 {write!(f, ", ")?;}
            // if count%2 == 0 { write!(f, " : ")?; }
            // else {write!(f, " , ")?;}
            // write!(f, "{}", v)?;

        }

        // 加上配对中括号，并返回一个 fmt::Result 值。
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
