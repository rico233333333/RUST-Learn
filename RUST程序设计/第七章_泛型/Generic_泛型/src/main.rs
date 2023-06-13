fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {  // 泛型类型
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    println!("Hello, world!");
}
