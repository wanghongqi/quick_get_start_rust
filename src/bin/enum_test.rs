#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6{ip:String},
}
fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6{ip:String::from("::1")};
    println!("{:?}",home);
    println!("{:?}",loopback);//loopback.ip不能访问，只能用match访问
    write_ip(&home);
    write_ip(&loopback);
    /**输出
    V4(127, 0, 0, 1)
    V6 { ip: "::1" }
    ipv4:127.0.0.1
    ipv6:::1
    **/
    println!("{}",get_ip(&home));
    println!("{}",get_ip(&loopback));
    /**输出
    ipv4:127.0.0.1
    ipv6:::1
    **/
    let str="abc";
    match str {
        "abc"=>println!("{}",str),
        _=>{}
    }//输出 abc
    let str=String::from("abc");
    match &str[..] {
        "abc"=>println!("{}",str),
        _=>{}
    }//输出 abc
}
//match输出枚举值内部的属性
fn write_ip(ip:&IpAddr){
    match ip {
        IpAddr::V4(a,b,c,d)=>{
            println!("ipv4:{}.{}.{}.{}",a,b,c,d)
        }
        IpAddr::V6{ip} => {
            println!("ipv6:{}",ip)
        }
    }
}
fn get_ip(ip:&IpAddr) -> String {
    let str:String= match ip {
        IpAddr::V4(a,b,c,d)=>{
            return format!("ipv4:{}.{}.{}.{}",a,b,c,d)
        }
        IpAddr::V6{ip} => {
            return format!("ipv6:{}",ip)
        }
    };
    str
}