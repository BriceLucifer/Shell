use std::env::{set_current_dir,current_dir};
use std::path::PathBuf;
use std::process::{exit,Command};
use std::io::{self,Write};
use colored::*;
// 编写一个简单的完备shell

type buildin_fntype = fn(Vec<String>) -> isize;

// 创建一个结构体
struct BuildIn{
    // 内置命令的字符串
    build_in_str: Vec<String>,
    // 内置命令的类型
    build_in_func: Vec<buildin_fntype>,
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
    println!("{}","-cd🌟\n-help🌌\n-exit✨\n-ls⚡".green());
    println!("{}","use man📔 command for more information on other programℹ️".green());
    return 1;
}

// ls函数
fn rsh_ls(args: Vec<String>) -> isize {
    let argc = args.len();
    let file_open = |path: Option<&str>| {
        let mut dir_path = PathBuf::new();
        match path {
            Some(exist) => dir_path.push(exist),
            None => match current_dir() {
                Ok(path) => dir_path = path,
                Err(e) => {
                    eprintln!("Error reading current directory: {}", e);
                    return;
                }
            },
        }
        match dir_path.read_dir() {
            Ok(dir) => {
                for entry in dir {
                    if let Ok(entry) = entry {
                        println!("{:?}", entry.file_name());
                    }
                }
            }
            Err(e) => eprintln!("Error reading directory '{}': {}", dir_path.display(), e),
        }
    };

    match argc {
        1 => file_open(None),
        2 => file_open(Some(&args[1])),
        _ => println!("rsh_ls accepts at most one argument"),
    }

    1
}

// exit函数
fn rsh_exit(_args:Vec<String>)->isize{
    exit(0);
}

// 启动函数
fn rsh_launch(args:&Vec<String>)->isize{
    if args.is_empty(){
        return 1;
    }
    let mut command = Command::new(&args[0]);
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
    // 环境参数
    let args = args_fun();
    for i in &args{
        println!("-{}",i);
    }

    let build_in_strs:Vec<String> = vec!["cd".to_string(),"help".to_string(),"ls".to_string(),"exit".to_string()];
    let buildin_funcs:Vec<buildin_fntype> = vec![rsh_cd,rsh_help,rsh_ls,rsh_exit];
    // 内置结构体
    let build_ins = BuildIn{
        build_in_str:build_in_strs.clone(),
        build_in_func:buildin_funcs.clone(),
    };
    loop{
        print!("{}","💕🦀rsh -> ".red().yellow());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let args: Vec<String> = input.split_whitespace().map(String::from).collect();
        if !args.is_empty() {
            rsh_execute(&args, &build_ins);
        }
    }

}