
// 定义结构体
// #[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 定义结构体方法
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 定义关联函数  下面函数指代 返回一个结构体对象
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
}

fn main() {
    let c = Rectangle {
        width: 32,
        height: 12,
    };

    println!("{}",c.height);

    println!("{}", c.area());

    use MY_PROJECT::eat_at_restaurant_;  // 只可以引用公有的 不可以引用私有的  use使用的是相对路径  可以使用as起别名
    eat_at_restaurant_();


}
