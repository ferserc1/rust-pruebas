
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("someone@example.com"),
        sign_in_count: 1
    };
    user1.email = String::from("user1@gmail.com");

    print_user(&user1);

    let user2 = build_user(String::from("paco"), String::from("paco@test.com"));
    print_user(&user2); 

    update_syntax();

    tuple_structs();

    rectangle_area_example();
}

fn print_user(user: &User) {
    println!("Active: {}, User name: {}, email: {}, sign in count: {}", user.active, user.username, user.email, user.sign_in_count);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 0
    }
}

fn update_syntax() {
    let user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@test.com"),
        sign_in_count: 1
    };

    // Método largo
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count
    };
    
    // Struct update syntax
    let user3 = User {
        email: String::from("user3@example.com"),
        ..user2
    };
    print_user(&user3);
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_structs() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Black color: {}, {}, {}", black.0, black.1, black.2);
    println!("Origin point: {}, {}, {}", origin.0, origin.1, origin.2);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn rectangle_area_example() {
    let rec1 = Rectangle {
        width: 30,
        height: 50
    };

    println!("The area of the rectangle {:?} is {} square pixels.", rec1, area(&rec1));

    dbg!(&rec1);

    println!("The area of the rectangle {:?} is {} square pixels.", rec1, rec1.area());

    if rec1.width() {
        println!("The rectangle {:?} has a width greater than 0.", rec1);
    }

    let rec2 = Rectangle {
        width: 10,
        height: 40
    };

    let rec3 = Rectangle {
        width: 60,
        height: 45
    };

    println!("Can rec1 hold rec2? {}", rec1.can_hold(&rec2));
    println!("Can rec1 hold rec3? {}", rec1.can_hold(&rec3));

    let square = Rectangle::square(3);
    println!("Square: {:?} is {} square pixels", square, square.area());
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}