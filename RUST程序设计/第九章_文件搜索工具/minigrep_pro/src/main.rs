use std::process;
use std::env;
use minigrep_pro;

fn main() {
    let args: Vec<String> = env::args().collect();

    // let config = Config::new(&args);  // 此行适用于new函数
    let config = minigrep_pro::Config::build(&args).unwrap_or_else(|err| {
        println!("错误请参见：{}", err);
        process::exit(1);  // 此处不在让手动panic!让程序崩溃 通过此行代码终止进程
    });

    println!("检索字符串：{}", config.query);
    println!("待检索文件：{}", config.file_path);

    // 文件读取
    // run(config);

    // 错误处理
    if let Err(e) = minigrep_pro::run(config) {
        println!("程序错误：{}", e);
        process::exit(1);
    }
}

// fn run(config:Config) -> Result<(), Box<dyn Error>>{
//     // 文件读取函数
//     let contents = fs::read_to_string(config.file_path)?;
//     println!("文件内容：\n {}",contents);
//
//     Ok(())
// }
//
// struct Config {
//     query: String,
//     file_path: String,
// }
//
// impl Config {
//     fn new(args: &[String]) -> Config {
//         if args.len() < 3 {
//             panic!("命令行输入的指令不符合规范");  // 自动panic! 让程序避免接下来的数组越界问题 直接让程序崩溃
//         }
//
//         let query = &args[1].clone();  // 此处对于字符串进行深拷贝
//         let file_path = &args[2].clone();
//
//         Config { query: query.to_string(), file_path: file_path.to_string() }
//     }
//
//     fn build(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("命令行输入指令不符合规范");
//         }
//
//         // let query = args[1].clone();
//         // let file_path = args[2].clone();
//         //
//         // Ok(Config { query, file_path })
//
//         Ok( Config {
//             query:args[1].clone().to_string(),
//             file_path:args[2].clone().to_string(),
//         } )
//     }
// }

