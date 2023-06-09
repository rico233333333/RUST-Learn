fn main() {
    // rust复合类型
    // tuple  数据类型可以不尽相同
    let a: (i8, i8, i8) = (1, 2, 3);
    println!("{}", a.0); // 元组取值 元组.元素下标

    // 获取tuple元素值
    // 我们可以使用模式匹配tuple来解构tuple
    let (x, y, z) = a; // 变量解构或者叫模式匹配
    println!("{},{},{}", x, y, z);

    // 访问tuple元素
    // 使用点标记法访问tuple的各个元素
    println!("{},{},{}", a.0, a.1, a.2);

    // 数组  数据类型相同的一组集合
    // 数组可以存放在stack栈内存上，而不是heap堆内存上 或者想保证固定的元素数量的元素使用数组更有好处
    // 数组没有vector灵活 由标准库组成
    let b: [i8; 4] = [1, 2, 3, 4];  // 定义了一串数组 类型是i8组成元素有4个
    // 特殊声明情况  声明元素相同数组 第一位是元素 第二位表示长度
    let _c = [3; 5];  // 这个数组有五个元素且每个元素都是3

    // 数组元素的访问 可以使用索引来访问数组元素
    let one = b[1];  // b的第二个元素
    println!("{}",one);

    // 超出数组的访问范围时
        /* 数组越界 会报 panic 错误*/

    let all = &b[..];  // b的所有元素  访问越界 程序会panicked产生恐慌 当然编译的时候无问题
    println!("{}",all[1]);

    // 数组的类型

}
