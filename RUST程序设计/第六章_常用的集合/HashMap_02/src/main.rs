use std::collections::HashMap;

fn main() {
    // HashMap值的更新
        // 如果向HashMap中插入一对KV然后在插入一对KV当然这两对也只是值不同 拥有相同的KEY 那后面插入的会替换前面插入的
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 26);

    println!("{:?}",scores);

    // 只在K不存在的时候插入键值对
        // .entry()方法
        // 上述方法配合or_insert()方法使用

    let e = scores.entry(String::from("Black"));
    println!("{:?}",e);  // 此处返回 Entry(VacantEntry("Black"))  此处返回空置入口 此方法检查是否有这个建的值
    e.or_insert(50);  // 若是上述值成立则插入键值对若是没有则不插入

    println!("{:#?}",scores);

    // 判断检测单词出现数量
    let text = "hello world wonderfun world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);  // 这串代码的返回类型是一个整形的引用 若是需要对其进行修改则需要解引用
        *count += 1;  // 解引用 修改了or_insert方法的返回值
    }

    println!("{:#?}", map);
}
