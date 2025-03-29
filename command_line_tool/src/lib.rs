use std::{env, error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //使用 Box<dyn Error> 作为统一的错误类型，使得函数可以返回任意实现了 Error trait 的错误。
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}

//struct来抽象
pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}
impl Config {
    //返回Result来处理
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        println!("{:?}", args);
        if args.len() < 3 {
            // panic!("Not enough arguments provided!"); //这样的话使用者会看到很多的信息 其实这些信息对用户来说是看不懂的 所以我们需要自定义错误信息 这个时候就需要用Result 然后在调用处处理异常和定义异常时的显示内容
            return Err("not enough arguments");
        }
        args.next(); //第一个位置
        // let query = &args[1];
        // let filename = &args[2];
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        }; //第二个位置
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        }; //第三个位置
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err(); //获取环境变量 命令行就可以是 CASE_INSENSITIVE=1 cargo run cargo README.md 表示设置环境变量后运行cargo run
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results
    //简化版本
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // let query = query.to_lowercase();
    // for line in contents.lines() {
    //     if line.contains(&query) {
    //         results.push(line);
    //     }
    // }
    // results

    //简写版本
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
        Rust:\
        safe, fast, productive.\
Pick three.\
Duct tape.";
        "
        ";
        assert_eq!(vec!["safe,fast,productive."], search(query, contents));
    }
}
