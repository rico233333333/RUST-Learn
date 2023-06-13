#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(CnState),
}

#[derive(Debug)]  // 此处得给每个枚举都添加Debug
enum CnState {
    GanSu,
    ShanXi,  // 我们允许被上一个枚举关联
}

fn value_in_cents(coin:Coin) -> u8 {
    // 此函数接受枚举
    match coin {
        // match相当于是拿值进行匹配的一个类型
        Coin::Penny => 1,
        Coin::Nickel => {
            println!("Nickel!");
            5
        },
        Coin::Dime => 10,
        // 绑定值的模式匹配
        Coin::Quarter(state) => {
            println!("{:?}!", state);
            25
        } ,
        // _=>
    }
}

fn main() {
    //
    let coin_1 = Coin::Penny;
    println!("{}", value_in_cents(coin_1));  // 看到这儿可以说match就是拿值进行匹配的

    let coin_2 = Coin::Quarter(CnState::ShanXi);
    println!("{}", value_in_cents(coin_2));  // 看到这儿可以说match就是拿值进行匹配的
}
