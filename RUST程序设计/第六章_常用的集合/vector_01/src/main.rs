fn main() {
    // 创建vector Vector存储在堆内存上 可以存储任何类型的数据
    let v:Vec<i32> = Vec::new();  // Vec::new()  创建空Vector没有元素

    // 以指定初始值的方式创建Vector
    let vec_:Vec<i32> = vec![1,2,3];

    // 更新Vector

    let mut vv = Vec::new();  // 此处创建一个空的Vector

    // 添加元素
    vv.push(1);  // 此处添加vector元素

    // 删除Vector
    // 当Vector走出作用域之后就会被清理掉 所有元素也会清理

    // 元素的访问
    let third:&i32 = &vec_[2];
    println!("{}",third);

    // 通过.get()方法 返回Option枚举  获取值 然后进行模式匹配堆值进行处理  get方法里边传递的参数是vector的索引

    match vec_.get(2) {  // 非法访问程序不会恐慌
        Some(third) => println!("{}",third),  // 之所以可以如此是因为它的返回值类型是Option
        None => println!("None")
    }

    // 通过for循环对vector数组进行便利
    // 定义Vector数组

    let v23 = vec![100,20,300];
    for i in &v23 {
        println!("{}",*i);  // 解引用
    }


    // 可变引用修改值
    let mut v234 = vec![100,20,300];
    for i in &mut v234 {
        *i += 50;
        // println!("{}",i);  // 解引用  此处修改了原vector
    }

    for i in &v234 {
        println!("{}",i);  // 解引用
    }

}
