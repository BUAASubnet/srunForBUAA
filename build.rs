use std::{env, net::Ipv4Addr};

fn main() {
    // 直接将 IP 地址硬编码，不再从环境变量读取
    let auth_server_ip = "10.200.21.4";

    println!("Using hardcoded AUTH_SERVER_IP = {}", auth_server_ip);
    
    // 解析硬编码的 IP 地址
    auth_server_ip
        .parse::<Ipv4Addr>()
        .expect("AUTH_SERVER_IP invalid");
}
