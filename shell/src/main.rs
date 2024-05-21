use std::env::{set_current_dir,current_dir};
use std::process::{exit,Command};
use std::io::{self,Write};
// 编写一个简单的完备shell

type buildin_fntype = fn(Vec<String>) -> isize;

// 创建一个结构体
struct buildIn{
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
fn rsh_help(args:Vec<String>)->isize{
    println!("Brice first shell in rust: rsh");
    println!("Type Program you want and hit <CR>");
    println!("The Following is buildin to rsh");
    println!("cd\nhelp\nexit");
    println!("use man commands for information on other program");
    return 1;
}

// exit函数
fn rsh_exit(args:Vec<String>)->isize{
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
        Ok(status) => {
            return 1;
        },
        Err(e) => {
            eprintln!("rsh: error executing command: {}",e);
            return 1;
        },       

    }
}

// 执行函数
fn rsh_execute(args:&Vec<String>,build:&buildIn)->isize{
    if args[0].is_empty(){
        return 1;
    }

    for i in 0..build.build_in_str.len(){
        if args[0].eq(&build.build_in_str[i]){
            return build.build_in_func[i](args.clone());
        }
    }
    return rsh_launch(args)
}

// 循环rsh函数
#[warn(dead_code)]
fn rsh_loop(){
    let mut line:String ;
    let mut args:Vec<String> ;
    let status : isize ;
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

    let mut build_in_strs:Vec<String> = vec!["cd".to_string(),"help".to_string(),"exit".to_string()];
    let mut buildin_funcs:Vec<buildin_fntype> = vec![rsh_cd,rsh_help,rsh_exit];
    // 内置结构体
    let mut build_ins = buildIn{
        build_in_str:build_in_strs.clone(),
        build_in_func:buildin_funcs.clone(),
    };
    loop{
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let args: Vec<String> = input.split_whitespace().map(String::from).collect();
        if !args.is_empty() {
            rsh_execute(&args, &build_ins);
        }
    }
    
}
