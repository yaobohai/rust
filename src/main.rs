fn say() -> String {
    return String::from("拜拜您👋");
}

fn main() {
    let user_name="博海";
    let user_status="😅";

    println!("你好啊,{}{} ",user_name,user_status);
    println!("------------------------------");

    // 调用say函数返回值
    let beybey = say();

    for i in 1..=5 {
        println!("这是第{}次 呵呵{} {}!",i,user_status,beybey)
    }
}
