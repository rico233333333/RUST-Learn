enum IpAddrKind {
    // 枚举使用 enum参数定义
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,  // struct中定义的枚举不可以拥有值 目前来说得使用结构体来实现
    address: String,
}

enum IpAddr_ {
    // 将值附加到枚举变体中  优点 不需要额外使用Struct  每个变体可以拥有不同的类型以及关联的数据量
    V4(u8,u8,u8,u8),
    V6(String),
    Move {x :i32, y :i32},
}

impl IpAddr_ {
    // 为枚举定义方法
    fn call(&self) {

    }
}


fn main() {
    let four = IpAddrKind::V4;  // 枚举值的使用定义与方法函数的调用方式类似 使用:: 两个冒号进行定义
    let sex = IpAddrKind::V6;

    // 使用函数进行创建枚举值
    route(four);
    route(sex);
    route(IpAddrKind::V4);  // 直接在函数变量里面创建枚举值

    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let _v_four = IpAddr_::V4(0,0,0,0);
    let m = IpAddr_::Move {x:32,y:64};
    // let _s = String::from("hello");
    let c = IpAddr_::V6(String::from("hello"));
    m.call();  // 此方法必须得定义完枚举类型的全部之后才可已使用
}

fn route(ip_kind: IpAddrKind) -> IpAddrKind {
    ip_kind
}
