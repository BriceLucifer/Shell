// 编写一个简单的完备shell rsh使用rust重写

// cd函数
fn rsh_cd(args:Vec<String>)->isize{
    return 1;
}

// help函数
fn rsh_help(args:Vec<String>)->isize{
    return 1;
}

// exit函数
fn rsh_exit(args:Vec<String>)->isize{
    return 0;
}

// 启动函数
fn rsh_launch(args:Vec<String>)->isize{
    return 1;
}

// 执行函数
fn rsh_execute(args:Vec<String>)->isize{
    return 1;
}

// 循环rsh函数
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
    for i in args{
        println!("-{}",i);
    }
}
