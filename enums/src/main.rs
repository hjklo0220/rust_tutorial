mod match_module;

enum IpAddKind {
    V4,
    V6,
}
enum IpAddrEnum {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddrStruct {
    kind: IpAddKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("message call");
    }
}

// enum Option<T> {
//     None,
//     Some(T),
// }

fn main() {
    match_module::match_module_main();

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap_or(0);
    println!("{sum}");

    let m = Message::Write(String::from("hello"));
    m.call();

    let four = IpAddKind::V4;
    let six = IpAddKind::V6;

    let home = IpAddrStruct {
        kind: IpAddKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddrStruct {
        kind: IpAddKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddrEnum::V4(127, 0, 0, 1);
    let loopback = IpAddrEnum::V6(String::from("::1"));


    route(IpAddKind::V4);
    route(IpAddKind::V6);

}

fn route(ip_kind: IpAddKind) {}