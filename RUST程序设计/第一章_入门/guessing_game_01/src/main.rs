use std::io;  // 导入标准库

fn main() {
    // 猜数字
    println!("猜数字！");

    println!("猜数字1-100");

    // 获取用户输入
    let mut guess = String::new();  // 定义了一个可变变量guess 绑定了一个空字符串

    io::stdin().read_line(&mut guess).expect("无法读取");

    println!("你猜测的数字是:{}",guess);
}
