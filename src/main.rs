use std::env;
use std::process;
use minigrep::Config;

//解析命令行参数
//初始化其他配置
//调用lib.rs中的run函数，以启动逻辑代码的运行
//如果run返回了一个错误，需要对该错误进行处理
fn main() {
    let args: Vec<String> = env::args().collect();

    // 对 build 返回的 `Result` 进行处理
    let config = Config::build(&args).unwrap_or_else(|err: &'static str| {
        println!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    if let Err(e) = minigrep::run(config) {
        println!("Application error is {e}");
        process::exit(1);
    }
}
