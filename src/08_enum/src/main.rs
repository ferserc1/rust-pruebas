enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move: {}, {}", x, y),
            Message::Write(s) => println!("Write: {}", s),
            Message::ChangeColor(r, g, b) => println!("ChangeColor: {}, {}, {}", r, g, b),
        }
    }
}


//enum IpAddr {
//    V4(String),
//    V6(String)
//}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    //let home = IpAddr::V4(String::from("127.0.0.1"));
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    route(home);
    route(loopback);

    let messages = [
        Message::Write(String::from("hello")),
        Message::Move { x: 1, y: 2 },
        Message::ChangeColor(1, 2, 3),
        Message::Quit,
    ];
    
    for message in messages.iter() {
        message.call();
    }

    let mut some_number: Option<u8> = Some(5);
    let mut some_string = Some("a string");
    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);

    some_number = None;
    some_string = None;
    println!("some_number: {:?}", some_number);
    println!("some_string: {:?}", some_string);

    let one_number: i8 = 6;
    let other_number: Option<u8> = Some(5);

    let result = match other_number {
        Some(n) => n,
        None => 0,
    } + one_number;
    println!("result: {}", result);

    if let Some(n) = other_number {
        println!("result: {}", n + one_number);
    }
}

fn route(_ip: IpAddr) {}

//enum IpAddrKind {
//    V4,
//    V6,
//}

//struct IpAddr {
//    kind: IpAddrKind,
//    address: String,
//}

//fn main() {
//    let home = IpAddr {
//        kind: IpAddrKind::V4,
//        address: String::from("127.0.0.1")
//    };

//    let loopback = IpAddr {
//        kind: IpAddrKind::V6,
//        address: String::from("::1")
//    };

//    route(home.kind);
//    route(loopback.kind);
//}

//fn route(_ip_kind: IpAddrKind) {}
