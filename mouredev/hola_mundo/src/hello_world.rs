use std::vec;
use std::collections::{ HashSet, HashMap };

fn main() {

    let mut my_string: &str = "Hello, World!";

    println!("{}", my_string);

    my_string = "Hello again, World!";
    println!("{}", my_string);

    // Las constantes no tienen inferencia de tipo
    const MY_CONST: i32 = 42;
    println!("{}", MY_CONST);

    let my_int: i32 = 10;

    // Control de flujo
    if my_int == 10 {
        println!("my_int es igual a 10");
    }
    else if my_int > 10 {
        println!("my_int es mayor a 10");
    }
    else {
        println!("my_int es menor a 10");
    }

    // Lista
    let mut my_list = vec!["Fernando","Serrano","ferserc1@gmail.com"];
    println!("Mi nombre es {} {} y mi correo es {}", my_list[0], my_list[1], my_list[2]);
    my_list[2] = "ferserc1@mac.com";
    println!("Mi nombre es {} {} y mi correo es {}", my_list[0], my_list[1], my_list[2]);
    my_list.push("42");
    println!("Mi nombre es {} {}, mi correo es {} y mi edad es {}", my_list[0], my_list[1], my_list[2], my_list[3]);

    // Set
    let mut my_set: HashSet<&str> = vec!["Fernando","Serrano","ferserc1@gmail.com"].into_iter().collect();
    my_set.insert("Fernando");
    println!("{:?}", my_set);
    my_set.insert("Antonio");
    println!("{:?}", my_set);
    my_set.insert("ferserc1@gmail.com");
    println!("{:?}", my_set);

    // Crea un hash map con los valores "name": "Fernando" y "last_name": "Serrano"
    let mut my_map: HashMap<&str, &str> = vec![
        ("name","Fernando"),
        ("last_name","Serrano"),
        ("email","ferserc1@gmail.com")].into_iter().collect();
    println!("{:?}", my_map);
    my_map.insert("email","ferserc1@mac.com");
    println!("{:?}", my_map);

    // Itera sobre el hash map
    for (key, value) in &my_map {
        println!("{}: {}", key, value);
    }

    // Itera sobre el set
    for value in &my_set {
        println!("{}", value);
    }

    // Itera sobre la lista
    for value in &my_list {
        println!("{}", value);
    }

    let mut my_counter = 0;
    while my_counter < 10 {
        println!("my_counter es {}", my_counter);
        my_counter += 1;
    }

    my_counter = 0;
    loop {
        println!("my_counter es {}", my_counter);
        my_counter += 1;
        if my_counter == 10 {
            break;
        }
    }

    my_function();
}

fn my_function() {
    println!("Esto es una función");

    let person = Person::new(String::from("Fernando"), String::from("Serrano"), String::from("ferserc1@gmail.com"));
    println!("{:?}", person);
    person.print_info();
}

#[derive(Debug)]
struct Person {
    pub name: String,
    pub last_name: String,
    pub email: String
}

impl Person {
    fn new(name: String, last_name: String, email: String) -> Person {
        Person {
            name,
            last_name,
            email
        }
    }

    fn print_info(&self) {
        println!("Name: {} {}, e-mail: {}", self.name, self.last_name, self.email);
    }
}