use std::env;
// use std::borrow::Cow;

fn say() -> String {
    return String::from("拜拜您👋");
}

pub fn get_env_var(name: &str) -> Option<String> {
    env::var(name).ok()
}

fn main() {
    // 这里从env中读取配置
    // export USER_NAME="张三"
    // export USER_STATUS="😢"
    // export EXEC_NUM="5"

    let user_name = get_env_var("USER_NAME").expect("环境变量USER_NAME 未定义");
    let user_status = get_env_var("USER_STATUS").expect("环境变量USER_STATUS 未定义");
    let exec_num = get_env_var("EXEC_NUM").expect("环境变量EXEC_NUM未定义");
    
    println!("你好啊,{}{} ",user_name,user_status);
    println!("------------------------------");

    // 调用say函数返回值
    let beybey = say();

    for i in 1..=exec_num.parse::<i32>().unwrap() {
        println!("这是第{}次 呵呵{} {}!",i,user_status,beybey)
    }
}
