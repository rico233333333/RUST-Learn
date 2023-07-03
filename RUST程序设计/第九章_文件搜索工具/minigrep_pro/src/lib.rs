use std::error::Error;
use std::process;
use std::env;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("命令行输入的指令不符合规范");  // 自动panic! 让程序避免接下来的数组越界问题 直接让程序崩溃
        }

        let query = &args[1].clone();  // 此处对于字符串进行深拷贝
        let file_path = &args[2].clone();

        Config { query: query.to_string(), file_path: file_path.to_string() }
    }

    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("命令行输入指令不符合规范");
        }

        // let query = args[1].clone();
        // let file_path = args[2].clone();
        //
        // Ok(Config { query, file_path })

        Ok( Config {
            query:args[1].clone().to_string(),
            file_path:args[2].clone().to_string(),
        } )
    }
}

pub fn run(config:Config) -> Result<(), Box<dyn Error>>{
    // 文件读取函数
    let contents = fs::read_to_string(config.file_path)?;
    println!("文件内容：\n {}",contents);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    vec![]
}