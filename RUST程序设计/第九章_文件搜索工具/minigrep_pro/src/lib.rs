use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    // 命令行传参结构体
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,  // 控制大小写
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("命令行输入的指令不符合规范");  // 自动panic! 让程序避免接下来的数组越界问题 直接让程序崩溃
        }

        let query = &args[1].clone();  // 此处对于字符串进行深拷贝
        let file_path = &args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Config {
            query: query.to_string(),
            file_path: file_path.to_string(),
            ignore_case,
        }
    }

    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("命令行输入指令不符合规范");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        // let query = args[1].clone();
        // let file_path = args[2].clone();
        //
        // Ok(Config { query, file_path })

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 文件读取函数
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

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

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive,
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

// 大小写敏感搜索函数
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

// 大小写不敏感搜索函数
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();  // 此处把字符串里面的大写字母全部转换成小写字母
    let mut results = Vec::new();  // 创建动态数组

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}