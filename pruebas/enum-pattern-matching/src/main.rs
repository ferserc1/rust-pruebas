use std::io::{ self, Write};


// Los enumerados se nombran en PascalCase, así como sus variantes
enum IpAddressKind {
    V4,
    V6
}

// Los enumerados pueden tener datos asociados
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String)
}

// Es posible combinar enumerados con datos asociados y sin datos asociados
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}

// Enumerado option: se puede usar para representar un valor que puede ser nulo
fn read_number_from_console() -> Option<i32> {
    print!("Enter a number: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    let trimmed = input.trim();
    match trimmed.parse() {
        Ok(number) => Some(number),
        Err(_) => None
    }
}

fn read_floating_point_from_console() -> Option<f32> {
    print!("Enter a floating point number: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    let trimmed = input.trim();
    match trimmed.parse() {
        Ok(number) => Some(number),
        Err(_) => None
    }
}

fn main() {
    let four = IpAddressKind::V4;
    let six = IpAddressKind::V6;

    route(four);
    route(six);

    let home = IpAddress::V4(127, 0, 0, 1);
    let loopback = IpAddress::V6(String::from("::1"));

    route_address(home);
    route_address(loopback);

    let quit = Message::Quit;
    let move_to_origin = Message::Move { x: 0, y: 0 };
    let write_hello_world = Message::Write(String::from("Hello, world!"));
    let set_blue = Message::ChangeColor(0, 0, 255);

    process_message(quit);
    process_message(move_to_origin);
    process_message(write_hello_world);
    process_message(set_blue);

    // Un Option es un enumerado, así que se puede procesar con match
    let number = read_number_from_console();
    match number {
        Some(n) => println!("Number: {}", n),
        None => println!("Invalid number")
    }

    // También se puede usar if let para simplificar el código
    if let Some(n) = read_floating_point_from_console() {
        println!("Number: {}", n);
    } else {
        println!("Invalid number");
    }

}

fn route(ip_kind: IpAddressKind) {
    match ip_kind {
        IpAddressKind::V4 => println!("V4"),
        IpAddressKind::V6 => println!("V6")
    }
}

fn route_address(ip_addr: IpAddress) {
    match ip_addr {
        IpAddress::V4(a, b, c, d) => println!("V4: {}.{}.{}.{}", a, b, c, d),
        IpAddress::V6(addr) => println!("V6: {}", addr)
    }
}

fn process_message(message: Message) {
    match message {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to ({}, {}, {})", r, g, b)
    }
}