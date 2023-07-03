use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();  // 此处的env::args()方法会读取分析传入的命令行参数  它读取到的第一个就是程序的可执行路径名
    /*
    此方法为获取命令行输入的指定命令
    cargo run -- test sample.txt
    -- 之前的是cargo run 启动这个项目
    -- 之后是用户在命令行输入的待检索命令
    */
    // dbg!(args);  // 打印获取到的参数

    // 存储读取到的参数
    let query = &args[1];  // 引用
    let file_path = &args[2];  // 引用

    println!("搜索字符串：{}", query);
    println!("待检索文件：{}", file_path);

    // 检索文件
    let contents = fs::read_to_string(file_path)
        .expect("检索不到指定文件");  // 错误处理

    println!("With text:\n{contents}");
}
