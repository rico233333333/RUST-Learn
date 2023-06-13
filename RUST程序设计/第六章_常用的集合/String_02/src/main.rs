fn main() {
    let a = String::from("0字符串的下标索引访问");
    println!("{}",&a.len());
    // println!("{}",&a[0]);  // 此行不成立
    // 不支持类似于vector数组的动态访问 RUST是UTF8编码格式1，只能访问到第一个编码

    // byte 计算机第一种看待Rust字符串的形式 或者说内存存储格式

    for byte in a.bytes() {  // 此处返回一个数组
        println!("{}",byte);
    }

    // unicode方式看待字符串
    for char in a.chars() {  // 此处返回一个数组
        println!("{}",char);  // 此处打印输出unicode标量值
    }

    // 字符串的切片
    let hello = "123hello";
    let s:&str = &hello[0..2];  // 若是切割字符串切割到某字符的某一个 byte值 不成立 会引发程序恐慌
    println!("{}",s);
}
