enum IpAddrKind {
    V4(String),
    V6(String),
}


let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// let home = IpAddr{
//     kind: IpAddrKind::V4,
//     address: String::from("127.0.0.1");
// };

// let loopback = IpAddr{
//     kind: IpAddrKind::V4,
//     address: String::from("::1");
// };

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V4(String::from("::1"));