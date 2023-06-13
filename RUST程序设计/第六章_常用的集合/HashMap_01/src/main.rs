use std::collections::HashMap;  // 导入哈希map

fn main() {
    let mut scores:HashMap<String,i32> = HashMap::new();  // 创建一个空的哈希map  并规定了它的建于值的类型

    let mut map = HashMap::new();

    map.insert("key".to_string(),12);  // 哈希map插入数据
    map.insert("key_1".to_string(),50);  // 哈希map插入数据

    // 另一种创建哈希map的方式 collect方法
    let teams = vec![String::from("Blue"),String::from("yellow")];
    let intial_map = vec![10,50];

    // 下一行代码解释 .iter()创建迭代返回他们的遍历器 适用于Vector数组里面 .zip()方法 创建元祖的数组 .collect()方法 创建哈希map
    let scored:HashMap<_,_> = teams.iter().zip(intial_map.iter()).collect();  // 此处给这个哈希map进行声明的时候 泛型填写下划线 是因为RUST可以自动推断出对应的数据类型


    // 面对哈希map的所有权转移
        // 对于String类型它的所有权会转移到哈希map中 对于定义在栈(stack)上的值则会进行浅拷贝 也就是直接复制到哈希map中

    let s1 = String::from("key1");
    let s2 = String::from("value1");

    let mut h_map = HashMap::new();

    h_map.insert(s1,s2);

    // println!("{},{}",s1,s2);  // 此处会报错 因为所有权已经转移到了h_map这个变量

    // 访问HashMap中的值
    // .get()方法 此方法类似于Vector中的 .get()方法 返回option枚举类型
    let mut scores_ = HashMap::new();

    scores_.insert(String::from("Blue"), 10);
    scores_.insert(String::from("Yellow"), 50);

    let key1 = String::from("Blue");
    let value_blue = scores_.get(&key1);  // 此处传递的参数表示借用了key1

    match value_blue {
        Some(s) => println!("{}",s),
        None => println!("没有匹配到值"),
    }

    // 使用for循环遍历hashMap
    for (k, v) in &scores_ {
        println!("{},{}",k,v);
    }
}
