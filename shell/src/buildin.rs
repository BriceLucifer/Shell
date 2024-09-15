use colored::*;
use std::env::{current_dir, set_current_dir};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::process::{exit, Command};

// 定义函数类型
pub type BuildinFntype = fn(Vec<String>) -> isize;

//处理内置命令结构体
pub struct BuildIn {
    // 内置命令的字符串
    pub build_in_str: Vec<String>,
    // 内置命令的类型
    pub build_in_func: Vec<BuildinFntype>,
}

// 退出函数
pub fn rsh_exit(_args: Vec<String>) -> isize {
    exit(0);
}

// 启动函数
pub fn rsh_launch(args: &Vec<String>) -> isize {
    // 如果没有参数启动 依旧返回1
    if args.is_empty() {
        return 1;
    }
    // 命令启动查找
    let mut command = Command::new(&args[0]);
    // 类型execvp()
    command.args(&args[1..]);
    match command.status() {
        Ok(_status) => {
            return 1;
        }
        Err(e) => {
            eprintln!(
                "{} {}",
                "rsh: error executing command:".bold(),
                e.to_string().bright_red().bold()
            );
            return 1;
        }
    }
}

// 执行函数
pub fn rsh_execute(args: &Vec<String>, build: &BuildIn) -> isize {
    if args[0].is_empty() {
        return 1;
    }

    // 循环匹配 如果是内置函数 就启动内置的
    for i in 0..build.build_in_str.len() {
        if args[0].eq(&build.build_in_str[i]) {
            // 其实就是个函数指针列表化
            return build.build_in_func[i](args.clone());
        }
    }
    // 启动函数 如果以上不是内置函数
    return rsh_launch(args);
}

// cd函数
pub fn rsh_cd(args: Vec<String>) -> isize {
    if args[1].is_empty() {
        eprintln!("rsh : expected arguments to be cd directory");
    } else {
        if set_current_dir(&args[1]).is_err() {
            eprintln!("rsh can not process");
        } else {
            println!("{:#?}", current_dir().unwrap());
        }
    }
    return 1;
}

// help函数
pub fn rsh_help(_args: Vec<String>) -> isize {
    println!("{}", "😘Brice first shell in rust🦀: rsh".green());
    println!("{}", "Type Program you want and hit ⌨️<CR>".green());
    println!("{}", "The Following command😊 is buildin to rsh".green());
    println!("{}", "-cd🌟\n-help🌌\n-exit✨\n-ls🌩️\n-cat🐱".green());
    println!(
        "{}",
        "use man📔 command for more information on other programℹ️".green()
    );
    return 1;
}

// ls函数
pub fn rsh_ls(args: Vec<String>) -> isize {
    let argc = args.len();
    let file_open = |path: Option<&str>| {
        let mut dir_path = PathBuf::new();
        // 如果目录存在 那就添加 不存在就添加本地目录地址
        match path {
            Some(exist) => dir_path.push(exist),
            None => match current_dir() {
                Ok(path) => dir_path = path,
                Err(e) => {
                    // 错误处理是否能够读取本地目录
                    eprintln!("Error reading current directory: {}", e);
                    return;
                }
            },
        }

        // 读取分支
        match dir_path.read_dir() {
            Ok(dir) => {
                for entry in dir {
                    if let Ok(entry) = entry {
                        // 后续添加功能 识别是不是目录还是文件 区分一下颜色
                        if let Ok(filetype) = entry.file_type() {
                            // 如果是目录就是蓝色 文件就是cyan色
                            if filetype.is_file() {
                                println!(
                                    "{}",
                                    entry.file_name().into_string().unwrap().bright_cyan()
                                );
                            } else {
                                println!(
                                    "{}",
                                    entry.file_name().into_string().unwrap().bright_blue()
                                );
                            }
                        } else {
                            eprint!("unable to read filetype");
                        }
                    }
                }
            }
            Err(e) => eprintln!("Error reading directory '{}': {}", dir_path.display(), e),
        }
    };

    // 匹配参数数量
    match argc {
        1 => file_open(None),           // 如果只是cd
        2 => file_open(Some(&args[1])), // 如果是cd + 路径
        _ => println!("ls accepts at most one argument"),
    }

    1
}

// cat函数
pub fn rsh_cat(args: Vec<String>) -> isize {
    let temp;
    if args.len() == 1 {
        println!("{}🐱: need more than one argument", &args[0]);
        return 1;
    } else if args.len() == 3 {
        temp = &args[2];
    } else {
        temp = &args[1];
    }

    let file = File::open(&temp);
    match file {
        Ok(opened) => {
            let mut buf_reader = BufReader::new(opened);
            let mut contents = String::new();

            // 判断参数
            if args.len() > 2 {
                let mut res = buf_reader.read_line(&mut contents).unwrap();
                for i in 1.. {
                    if res != 0 {
                        println!(
                            "{} {}",
                            i.to_string().bright_magenta(),
                            contents.trim_end().to_string().bright_cyan()
                        );
                        contents.clear();
                        res = buf_reader.read_line(&mut contents).unwrap();
                    } else {
                        break;
                    }
                }
            } else {
                let res = buf_reader.read_to_string(&mut contents);
                match res {
                    Ok(number) => println!("{}{}", "total number is: ".cyan(), number),
                    Err(e) => eprintln!("error read_to_string {}", e),
                }
                println!("{}", contents.to_string().green());
            }
        }
        Err(_e) => {
            eprintln!("{} {}", "cat🐱: can not open file:".bold(), &args[1]);
        }
    }
    return 1;
}

pub fn rsh_wc(args: Vec<String>) -> isize {
    if args.len() != 2 {
        return 1;
    }

    let file = File::open(&args[1]);

    let lens = |x: &mut File| -> usize {
        x.seek(SeekFrom::Start(0)).unwrap();
        let bufferfile = BufReader::new(x);
        let mut linecount = 0;
        for _ in bufferfile.lines() {
            linecount += 1;
        }
        return linecount;
    };

    let word_count = |x: &mut File| -> usize {
        x.seek(SeekFrom::Start(0)).unwrap();
        let mut bufferfile = BufReader::new(x);
        let mut contents = String::new();

        let _ = bufferfile.read_to_string(&mut contents);
        let word_count = contents.split_whitespace().count();
        return word_count;
    };

    let bytesize = |x: &str| -> u64 {
        let bufferfile = fs::metadata(x).unwrap();
        let file_size = bufferfile.len();
        return file_size;
    };

    match file {
        Ok(mut opened) => {
            let lines = lens(&mut opened);
            let words = word_count(&mut opened);
            let bytes = bytesize(&args[1]);
            println!("{} {} {} {}", lines, words, bytes, &args[1].cyan())
        }
        Err(e) => eprintln!("Error open : {}", e),
    }

    return 1;
}
