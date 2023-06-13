fn main() {
    // 切片
    // 一串代码说明字符串有效性
    // 一旦字符产发生更改 那下文无法做出对应的更正 解决此问题的最优法便是字符串切片
    let mut s = String::from("Hello World");
    let u = first_world(&s);
    println!("{}",u);
    s.clear();  // 清空字符串

    // 字符串切片 -> 指向一部分字符的字符串引用
    // 字符串切片的形式 :[start..stop)  stop取不到
    let s2 = String::from("hello world");
    let hello:&str = &s2[0..5];  // 此处5取不到 左取右不取
    let world:&str = &s2[6..];
    println!("{} {}",hello,world)
}

fn first_world(s:&String) -> usize {
    let bytes = s.as_bytes();

    for (i,&item) in bytes.iter().enumerate() {  // 此处iter()方法返回一个迭代器(依次返回每个元素) 此处enumerate()方法(对上面迭代器返回的每个元素进行包装返回对应元组的一部分，返回的第一个值就是元素索引 第二个值就是迭代器返回的那个值)
        // 此处判断 判断字符串的字面值
        if item == b' ' {
            return i
        }
    }
    s.len()
}
