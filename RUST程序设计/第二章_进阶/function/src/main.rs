fn main() {
    println!("Hello, world!");
    let a = 6;
    ms(a);

    // 语句与表达式
        // 语句带封号
        // 表达式不带封号
    // 举个例子
    let y = {
        let x = 5 + a;
        5 + x  // 因为这里没有添加封号 故 此处类型是 自动识别的 i8
    };

    let m = {
        5 + 6;  // 因为这里添加了封号 故 此处类型是tuple () 类型
    };
}

fn ms(x: i8) -> () {  // 形参  函数参数必须指明
    println!("hello,{}",x);
}
