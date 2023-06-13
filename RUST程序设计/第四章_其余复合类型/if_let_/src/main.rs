fn main() {
    // if_let
    // 简单的控制流

    let v = Some(0u8);
    // 复杂的控制流(并不是说不好而是太庞大了)
    match v {
        Some(3) => println!("{}","Three"),
        _=> println!("other"),
    }

    // 简单的控制流
    if let Some(3) = v {
        // 放弃了某些东西
        println!("Three");
    } else {
        println!("other");
    }
}
