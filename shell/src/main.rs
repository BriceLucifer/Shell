mod buildin;
use buildin::*;
mod argsenv;
use argsenv::args_fun;
use colored::*;
use std::io::{self,Write};

fn main(){

    // 初始界面
    println!("{}","a shell written in rust🦀 By Brice✨".bright_cyan());
    println!("------------------------------------------");
    println!("start your shell journey😊");
    println!("\n");

    // 环境参数
    let _args = args_fun();

    let build_in_strs:Vec<String> = vec!["cd".to_string(),"help".to_string(),"ls".to_string(),"cat".to_string(),"wc".to_string(),"exit".to_string()];
    let buildin_funcs:Vec<BuildinFntype> = vec![rsh_cd,rsh_help,rsh_ls,rsh_cat,rsh_wc,rsh_exit];
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
