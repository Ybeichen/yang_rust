use std::env;
use std::process;
use minigrep::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {  // 使用该函数可以自定义非panic，同时因为返回的是结构体，所以需要解包
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);  // 错误情况下不会再有额外大量的输出
    });

    // --snip--
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {  // ？只在错误的时候返回 Err 类型，成功时返回(),所以不需要解包
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

