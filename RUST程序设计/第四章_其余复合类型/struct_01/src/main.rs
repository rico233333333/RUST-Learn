struct Rectangle {
    width: u32,
    length: u32,
}

struct Rect(u32, u32);

impl Rectangle {
    // 定义方法 结构体方法 方法名必须和结构体名相同
        // 方法的第一个参数可以是&self(不可变借用这样写代表不获取其所有权 若是要获取所有权那可以写 self) 也可以获得其所有权 或 可变借用，和其他参数一样
    // 此处给Rectangle枚举定义方法
    // 在此处self就相当于枚举类型
    fn area_struct(&self) -> u32 {
        // 此方法引用不获取所有权 也可以获取所有权
        self.length * self.width
    }

    fn can_hold(&self, other:&Rectangle) -> bool {
        // 判断一个长方形能否容纳另一个长方形
        self.width > other.width && self.length > other.length
    }

    fn square(size: u32) -> Rectangle {
        // 关联类型或关联函数
        // 这是一个关联类型 如何调用？ -> Rectangle::square(size)  其实是类型名::函数即可
        // 和String::from() 一样
        Rectangle {
            width: size,
            length: size,
        }
    }
}

fn main() {
    // struct小例子 计算长方形面积
    let w = 30;
    let l = 50;
    println!("----------{}:{}----------", 1, "area");
    println!("{}", area(w, l));

    println!("----------{}:{}----------", 2, "area_tuple");
    let rect = (w, l);
    println!("{}", area_tuple(rect));

    println!("----------{}:{}----------", 3, "area_struct");
    let rect = Rectangle {
        width: 32,
        length: 32,
    };
    println!("{}", area_struct(&rect));

    println!("----------{}:{}----------", 4, "area_tuple_struct");
    let rect_ = Rect(23, 21);
    println!("{}", area_tuple_struct(&rect_));

    println!("----------{}:{}:{}----------", 5, "area_impl_struct", "第一种写法");
    let rect_ = Rect(23, 21);
    println!("{}", Rectangle::area_struct(&rect));

    println!("----------{}:{}:{}----------", 5, "area_impl_struct", "第二种写法");
    let rect_ = Rect(23, 21);
    println!("{}", rect.area_struct());

    println!("----------{}:{}----------", 6, "Rectangle::can_hold");
    let rect1 = Rectangle {
        width:30,
        length:30,
    };
    let rect2 = Rectangle {
        width:60,
        length:40,
    };
    let rect3 = Rectangle {
        width:90,
        length:50,
    };
    println!("{}",rect1.can_hold(&rect2));

    println!("----------{}:{}:{}----------", 7, "Rectangle::square", "关联方法调用");
    let square = Rectangle::square(9);
    println!("{}", square.length);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dim: (u32, u32)) -> u32 {
    dim.0 * dim.1
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.length * rect.width
}

fn area_tuple_struct(rect: &Rect) -> u32 {
    rect.0 * rect.1
}

// fn area


