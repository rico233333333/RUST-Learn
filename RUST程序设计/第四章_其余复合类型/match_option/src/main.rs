fn main() {
    let five = Some(5);
    let _sex = plus_one(five);
    let _none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1), // 此处是在Some枚举内部进行处理的
    }
}
