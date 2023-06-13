fn main() {
    // 创建一个新字符串
    // 创建空字符串
    let mut str_ = String::new();  // 创建一个空字符串

    // 使用初始值来创建String  此处使用to_string()方法

    let a = "123阿达是的·".to_string();

    // String的增加
    let mut s = String::from("asd");
    s.push_str("123");  // 此处传递的参数其实借用了str类型  此处也可以传递一个引用
    s.push_str(&a);  // 类似于此处就传递了一个引用

    println!("{}",s);

    // 字符串的拼接
    // 使用+号拼接  要求 +号之前 放置一个字符串 后边放置一个字符串切片的引用

    let s1 = String::from("使用加号对字符串进行拼接");

    let s2 = String::from("这是被拼接字符串");

    let s3 = s1 + &s2;  // 此处第二个参数也就是+号后面的参数必须是引用

    println!("{}",s3);

    // fn add(self, &str) -> String {} 他实际上类似于这样

    // format例子 格式化字符串
    let q1 = String::from("字符串1");
    let q2 = String::from("字符串2");
    let q3 = String::from("字符串3");

    let q = format!("{}-{}-{}",q1,q2,q3);  // 此处不对以上三个变量进行借用

    println!("{}",q);


}


