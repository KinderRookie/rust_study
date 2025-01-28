fn main() {

    let four = IpAddrKind::V4;
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    let home = IpKind::V4(127, 0, 0, 1);
    let loopback = IpKind::V6(String::from("::1"));

    let s = Message::Write(String::from("hello"));
    s.call();
}

enum IpAddrKind {
    V4(String),
    V6(String),
} // 열거형의 variants 에 타입을 묶는 방법

enum IpKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
} // 구조체 안에 열거형을 넣는 방법

fn route(ip_type: IpAddrKind) {
}


// struct vs enumeration
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

/*
 열거형이 구조체와 다른 점은
 struct 키워드를 사용하지 않는다는 것과
 모든 배리언트가 Message 타입으로 묶인다는 것
 */

struct QuitMessage;
struct MoveMessage {
    x: i32, y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

// 마찬가지로 impl 가능하다.
impl Message {
    fn call(&self) {}
}
