enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    // can put any kind of data inside an enum variant:
    // strings, numeric types, or structs, for example.
    // You can even include another enum
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');

    // let absent_number: Option<i32> = None;

    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);

    // let sum = x + y;
}

fn route(ip_kind: IpAddrKind) {}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

enum Option<T> {
    None,
    Some(T),
}
