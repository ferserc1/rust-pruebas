
/// Box: almacenar en el heap
fn box_example_1() {
    // Box es un smart pointer que almacena datos en el heap
    let b = Box::new(5);
    println!("b = {}", b);
    let c = Box::new(10);
    println!("c = {}", c);

    // Con el operador * se accede al valor almacenado en el heap
    let d = *b + *c;
    println!("d = {}", d);

    // Al finalizar la función, los valores almacenados en el heap son liberados automáticamente
}

// Con Box podemos crear estructuras recursivas
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn box_example_2() {
    let list = List::Cons(1, 
        Box::new(List::Cons(2, 
        Box::new(List::Cons(3, 
        Box::new(List::Nil))))));

    println!("{:?}", list);
}

// Internamente, Box es un puntero inteligente que implementa el trait Deref
struct MyBox<T>(T);

impl <T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn box_example_3() {
    let y = MyBox::new(6);
    assert_eq!(6, *y);  // Con el rasgo Deref podemos acceder al valor almacenado en el heap
}

// Para el siguiente ejemplo necesitamos una función que reciba una referencia a un string slice
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn box_example_4() {
    // Rust puede convertir de forma automática una referencia a un tipo que implemente Deref, en una referencia a otro tipo. A esto se le llama coherción de tipos
    let m = MyBox::new(String::from("Rust"));

    // Esto es lo que tendríamos que hacer si Rust no implementara la coherción de tipos
    // *m -> Primera dereferencia, obtenemos una referencia a un String. Esto se puede hacer porque hemos implementado el trait Deref
    // &(*m) -> El operador & aquí hace una referencia a la referencia obtenida en el paso anterior.
    // &(*m)[..] -> Finalmente, obtenemos el string slice
    hello(&(*m)[..]);

    // Mediante la coherción de tipos, Rust es capaz de hacer todo esto de forma automática, haciendo coincidir el tipo de la referencia que 
    // espera la función hello con el tipo de la referencia que le pasamos. Esto se consigue gracias a que tanto MyBox como String implementan el trait Deref
    hello(&m);
}

// Para referencias mutables, Rust implementa el trait DerefMut.

// El rasgo Drop se utiliza para liberar recursos cuando un valor sale del ámbito
#[derive(Debug)]
struct CustomSmartPointer {
    data: String
}

// Implementamos Deref para poder acceder al valor almacenado en el heap en la macro println!
impl std::ops::Deref for CustomSmartPointer {
    type Target = String;

    fn deref(&self) -> &String {
        &self.data
    }
}

// Implementamos Drop para liberar los recursos cuando el valor sale del ámbito. En este caso, imprimimos un mensaje que se verá al salir del ámbito
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}

fn box_example_5() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers {} and {} created.", *c, *d);


    // Rust libera los recursos automáticamente cuando los valores salen del ámbito
    // En este caso, los valores c y d son liberados al finalizar la función
}

fn box_example_6() {
    // Podemos borrar un valor del heap de forma manual con la función drop
    let c = CustomSmartPointer { data: String::from("my stuff") };
    println!("{}", *c);

    std::mem::drop(c);

    println!("CustomSmartPointer dropped before the end of the function");
}

fn main() {
    box_example_1();
    box_example_2();
    box_example_3();
    box_example_4();
    box_example_5();
    box_example_6();
}
