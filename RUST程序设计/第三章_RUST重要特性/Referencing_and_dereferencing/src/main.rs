fn main() {
    // 可变引用与不可变引用
    // 规则 可变引用只能出现一个 不可变引用可以出现多个想出现几个就出现几个
    let mut a = 4;
    let b = &mut a;  // 可变引用只可以出现一次 引用又可以称为借用  引用不代表拥有
    let c = &mut a;
    a += 1;
    println!("{}",a);

    // let mut str_ = String::from("这是一个字符串哦");
    // let d = &mut str_;
    // let e = &mut str_;
    // println!("{},{}",d, e);  // 可变引用只能存在一个  不可变引用可以无限制的声明


    // 不可变引用
    let string_ = String::from("这是一个字符串哦");
    let f = &string_;
    let g = &string_;
    println!("{},{}",f,g);
}
