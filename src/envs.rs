// 读取系统环境变量信息
use std::env;
use std::borrow::Cow;


// 直接读取时
fn firest() {    
    let app_path = "APP_PATH";

    match env::var(app_path) {
        Ok(val) => println!("{app_path}: {val:?}"),
        Err(e) => println!("[ERROR] {app_path}: {e}"),
    }

    let app_port = "APP_PORT";
    match env::var(app_port) {
        Ok(val) => println!("{app_port}: {val:?}"),
        Err(e) => println!("[ERROR] {app_port}: {e}"),
    }
}

// 作为公共函数被调用时
pub fn get_env_var(name: &str) -> Option<String> {
    env::var(name).ok()
}

fn main() {
    
    firest();
    default_env();

    let app_conf_path = get_env_var("APP_CONF_PATH").expect("env APP_CONF_PATH 未定义");

    println!("app conf path: {}", app_conf_path);
}