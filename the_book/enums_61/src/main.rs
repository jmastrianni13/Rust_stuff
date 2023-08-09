fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call()
}

fn route(ip_kind: IpAddrKind) {}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor( i32, i32, i32 ),
}

impl Message {
    fn call(&self) {
        // method body woudl be defined here
    }
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum IpAddrKind {
    V4,
    V6
}

