use std::simd::i32x2;

#[derive(Debug)]  // 此处得引入debug因为我想打印结构体
// 定义结构体
struct User {
    username: String,
    // 规定数据类型 每个元素之间使用逗号隔开
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {  // 一旦此处声明的struct的实例是可变的 那实例中的所有字段都是可变的 struct可以作为函数的返回值
        email: String::from("额么么么么"),
        active: true,
        sign_in_count: 788,
        username: String::from("海海海"),
    };
    // 若是想要对元素进行访问 得使用点标记法对元素进行访问
    println!("{}", user1.username);
    // 若是想给某字段赋新值 则 必须声明 对应的结构体变量必须可变 否则 报错
    user1.email = String::from("168@qq.com");
    println!("{}", user1.email);



    let user2 = User {  // 一旦此处声明的struct的实例是可变的 那实例中的所有字段都是可变的 struct可以作为函数的返回值
        email: String::from("额么么么么"),
        active: true,
        sign_in_count: 788,
        username: String::from("海海海"),
    };

    // println!("_____________");
    // let b = struuu(user2);
    // println!("{}", b.email);



    let c = User {
        email:String::from("12"),
        username:String::from("12987987"),
        ..user2  // 结构体更新语法 避免写过多的代码类似于TypeScript 因为声明的结构体都存储在栈内存上 故 可以直接复制 或者采用更加专业的说法叫做浅拷贝
    };
    println!("_____________");
    println!("{}",c.email); // 结构体创建成功哦

    // 打印结构体信息必须使用debug  其中使用debug的时候得在println!("{:?}",结构体对象)
    println!("{:?}",user2)

}

// fn struuu(u:User) -> User {
//     // 此函数 脱裤子放屁白办手续
//     // struct作为函数返回值的例子
//     u
// }

fn build_user(email: String, username: String) -> User {
    User {  // 当rust struct中某字段与形参名称相同时可以进行简写
        // 简写前
        // email: email,
        // username: username,

        // 简写后
        email,
        username,
        active: true,
        sign_in_count: 0,
    }
}

fn tuple_struct() -> (){
    // 元组结构体
    // 定义元组结构体
    struct Color(i32, i32, i32);
    let black = Color(0,0,0);
}
