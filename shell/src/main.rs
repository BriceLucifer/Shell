use std::env::{set_current_dir,current_dir};
use std::fs::File;
use std::path::PathBuf;
use std::process::{exit,Command};
use std::io::{self, BufReader, Read, Write};
use colored::*;
// 编写一个简单的完备shell

type BuildinFntype = fn(Vec<String>) -> isize;

// 创建一个结构体
struct BuildIn{
    // 内置命令的字符串
    build_in_str: Vec<String>,
    // 内置命令的类型
    build_in_func: Vec<BuildinFntype>,
}

// cd函数
fn rsh_cd(args:Vec<String>)->isize{
    if args[1].is_empty(){
        eprintln!("rsh : expected arguments to be cd directory");
    }else{
        if set_current_dir(&args[1]).is_err(){
            eprintln!("rsh can not process");
        }else{
            println!("{:#?}",current_dir().unwrap());
        }
    }
    return 1;
}

// help函数
fn rsh_help(_args:Vec<String>)->isize{
    println!("{}","😘Brice first shell in rust🦀: rsh".green());
    println!("{}","Type Program you want and hit ⌨️<CR>".green());
    println!("{}","The Following command😊 is buildin to rsh".green());
    println!("{}","-cd🌟\n-help🌌\n-exit✨\n-ls🌩️\n-cat🐱".green());
    println!("{}","use man📔 command for more information on other programℹ️".green());
    return 1;
}

// ls函数
fn rsh_ls(args: Vec<String>) -> isize {
    let argc = args.len();
    let file_open = |path: Option<&str>| {
        let mut dir_path = PathBuf::new();
        // 如果目录存在 那就添加 不存在就添加本地目录地址
        match path {
            Some(exist) => dir_path.push(exist),
            None => match current_dir() {
                Ok(path) => dir_path = path,
                Err(e) => { // 错误处理是否能够读取本地目录
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
                        if let Ok(filetype) = entry.file_type(){
                            // 如果是目录就是蓝色 文件就是cyan色
                            if filetype.is_file() {
                                println!("{}",entry.file_name().into_string().unwrap().bright_cyan());
                            }else {
                                println!("{}",entry.file_name().into_string().unwrap().bright_blue());
                            }
                        }else {
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
        1 => file_open(None),   // 如果只是cd
        2 => file_open(Some(&args[1])),     // 如果是cd + 路径
        _ => println!("rsh_ls accepts at most one argument"),
    }

    1
}

// cat函数
fn rsh_cat(args:Vec<String>)->isize{
    if args.len() == 1{
        println!("{}🐱: need more than one argument",&args[0]);
        return 1;
    }

    let file = File::open(&args[1]);
    match file {
        Ok(opened) => {
            let mut buf_reader = BufReader::new(opened);
            let mut contents = String::new();
            let res = buf_reader.read_to_string(&mut contents);
            match res {
                Ok(num) => println!("😊the number of contents: {}",num),
                Err(e) => eprintln!("cat🐱: error read_to_string() {}",e),
            }
            println!("{}",contents.bright_green().bold());
        },
        Err(_e) => {
            eprintln!("{} {}","cat🐱: can not open file:".bold(),&args[1]);
        }
    }
    return 1;
}

// exit函数
fn rsh_exit(_args:Vec<String>)->isize{
    exit(0);
}

// 启动函数
fn rsh_launch(args:&Vec<String>)->isize{
    // 如果没有参数启动 依旧返回1
    if args.is_empty(){
        return 1;
    }
    // 命令启动查找
    let mut command = Command::new(&args[0]);
    // 类型execvp()
    command.args(&args[1..]);
    match command.status(){
        Ok(_status) => {
            return 1;
        },
        Err(e) => {
            eprintln!("{} {}","rsh: error executing command:".bold(),e.to_string().bright_red().bold());
            return 1;
        },

    }
}

// 执行函数
fn rsh_execute(args:&Vec<String>,build:&BuildIn)->isize{
    if args[0].is_empty(){
        return 1;
    }

    // 循环匹配 如果是内置函数 就启动内置的
    for i in 0..build.build_in_str.len(){
        if args[0].eq(&build.build_in_str[i]){
            // 其实就是个函数指针列表化
            return build.build_in_func[i](args.clone());
        }
    }
    // 启动函数 如果以上不是内置函数
    return rsh_launch(args)
}


// 环境变量函数
fn args_fun()->Vec<String>{
    let args:Vec<String> = std::env::args().collect();
    args
}

fn main(){

    //clap
    println!("{}","a shell written in rust🦀 By Brice✨".bright_cyan());
    println!("------------------------------------------");
    println!("start your shell journey😊");
    println!("\n");

    // 环境参数
    let _args = args_fun();

    let build_in_strs:Vec<String> = vec!["cd".to_string(),"help".to_string(),"ls".to_string(),"cat".to_string(),"exit".to_string()];
    let buildin_funcs:Vec<BuildinFntype> = vec![rsh_cd,rsh_help,rsh_ls,rsh_cat,rsh_exit];
    // 内置结构体
    let build_ins = BuildIn{
        build_in_str:build_in_strs.clone(),
        build_in_func:buildin_funcs.clone(),
    };
    // 循环一直运行 类似之前c语言单独我写的bsh_loop()
    loop{
        print!("{}","💕🦀rsh -> ".red().yellow());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim().is_empty(){
            continue;
        }
        let args: Vec<String> = input.split_whitespace().map(String::from).collect();
        if !args.is_empty() {
            rsh_execute(&args, &build_ins);
        }
    }

}