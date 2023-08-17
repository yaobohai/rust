fn say() -> String {
    return String::from("拜拜您👋");
}

fn main() {    
    let status="😅";
    let user_name="博海";

    println!("你好啊,{}{} ",user_name,status);
    println!("------------------------------");

    // 调用say函数返回值
    let beybey = say();

    for i in 1..=5 {
        println!("这是第{}次 呵呵{} {}!",i,status,beybey)
    }
}
