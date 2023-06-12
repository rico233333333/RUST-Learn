use std::io;  // 导入标准库
use rand::Rng;  // 随机数生成器
use std::cmp::Ordering;  // 导入标准库里面的枚举

fn main() {
    // 猜数字2.0
    println!("猜数字！");

    let secret_number = rand::thread_rng().gen_range(1..101); // 生成1到100之间（包括1和100）的随机数

    println!("猜数字1-100");

    // 获取用户输入
    let mut guess = String::new();  // 定义了一个可变变量guess 绑定了一个空字符串

    io::stdin().read_line(&mut guess).expect("无法读取");

    println!("你猜测的数字是:{}",guess);

    println!("编译器生成的数字是:{}",secret_number);

    let guess: u32 = guess.trim().parse().expect("解析出错 请输入数字");

    // 此处创建匹配模式
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("太小了"),
        Ordering::Equal => println!("你赢了猜对了"),
        Ordering::Greater => println!("太大了"),
    }
}
