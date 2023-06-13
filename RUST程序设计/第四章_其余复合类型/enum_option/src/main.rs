// 标中库中的定义  包含在预导入模块中可以直接使用
// enum Option<T> {
//     Some(T),
//     None,
// }

// 我们可以直接使用
// Option<T>
// -Some(T)
// -None

fn main() {
    // option 枚举 存在与标中库中
    let some_number = Some(5);  // 他的类型是 Option<i8> 类型 编译器会自动推断哦
    let some_string = Some("A String");

    let absent_number: Option<i32> = None;  // 创建None系统无法自动推断

    // 要想使用Option<T> 中的T，先必须把它转换为T
}
