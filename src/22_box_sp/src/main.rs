
fn box_1() {
    let b = Box::new(5);
    println!("b = {}", b);
}

// Usar Box<T> para almacenar datos en el heap y romper la recursividad al definir el tamaño de la estructura
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil
}

fn box_2() {
    let list = List::Cons(1,
        Box::new(List::Cons(2,
        Box::new(List::Cons(3,
        Box::new(List::Nil))))));
    println!("{:?}", list);
}

fn main() {
    box_1();
    box_2();
}
