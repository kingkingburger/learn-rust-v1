enum IpAddrKind {
    V4(u8,u8,u8,u8),
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

let home = IpAddr::V4(127,0,0,1);

let loopback = IpAddr::V4(String::from("::1"));


enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // 유닛 구조체
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 튜플 구조체
struct ChangeColorMessage(i32, i32, i32); // 튜플 구조체
