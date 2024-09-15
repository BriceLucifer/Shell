// 处理环境参数

// 环境变量函数
pub fn args_fun() -> Vec<String> {
    let args: Vec<String> = std::env::args().collect();
    args
}
