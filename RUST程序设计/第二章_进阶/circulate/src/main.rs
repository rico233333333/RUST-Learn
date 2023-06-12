fn main() {
    // loop循环
    // loop循环代表一直反复执行某段代码知道自己喊停为止
    let a:isize = 10000;
    // loop {
    //     println!("这是一个死循环")
    // }
    let mut _i = 0;

    // loop {
    //     println!("{}",i);
    //     if i == a {
    //         println!("循环跳出了");
    //         break;
    //     }
    //     i += 1;
    // }

    // while 条件循环
    // while i <= a {
    //     println!("不满足{}",i);
    //     i += 1;
    // }
    // println!("结束");

    // for循环遍历
    // let a = [10,20,30,50,98];
    // for j in a {
    //     println!("{}",j)
    // }
    //
    // // 加强版
    // for element in a.iter() {  // a.iter() 此方法生成可迭代对象 经常用于遍历对象
    //     println!("{}",element);
    // }

    // Range标准库提供
    // 需要指定一个开始与一个结束的数字  Range可以生成一个不包含结束的之间的数字
    // rev方法可以反转Range

    for number123 in (1..4).rev() {
        println!("{}",number123);
    }
}
