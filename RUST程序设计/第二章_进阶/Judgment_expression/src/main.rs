fn main() {
    let number = 3;

    if number < 5 {
        println!("太小了");
    } else if number == 5 {
        println!("哎哟不错哟");
    } else {
        println!("害，你输的太大了");
    }

    // 使用match语句进行重构
    match number {
        n if n > 5 => println!("太小了"),
        _ => println!("还行"),
    };

    // if 语句作为值的形式

    let bool = true;
    let number_ = if bool {5} else {6};
    println!("{}",number_);
}
