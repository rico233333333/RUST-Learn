enum Spr {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    let row = vec![
        Spr::Int(12),
        Spr::Float(23.12),
        Spr::Text(String::from("我也不知道")),
    ];
}
