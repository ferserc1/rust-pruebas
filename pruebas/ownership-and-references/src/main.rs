
// Reglas de posesión en Rust:
// 1. Cada valor en Rust tiene un propietario
// 2. Solo puede haber un propietario a la vez de un mismo valor
// 3. Cuando el propietario sale del ámbito, el valor se libera

// Reglas de referenica:
// 1. En un instante dado, solo se puede tener o bien una referencia mutable o bien cualquier número de referencias inmutables
// 2. Las referencias deben ser válidas, es decir, no pueden apuntar a un valor que ya no existe

fn main() {
    {
        // s es un String dinámico que se reserva en el heap
        let mut s = String::from("hello");
        println!("{}", s);

        s.push_str(", world!");
        println!("{}", s);
    } // s sale del ámbito y su contenido se libera del heap, porque el valor deja de tener propietario

    let ss: String;
    {
        let s = String::from("hello");
        ss = s;  // s se mueve a ss. Ahora el propietario de "hello" es ss

        // println!("{}", s); // Error: s ya no es propietario de "hello"

    } // s sale del ámbito, pero ss sigue siendo propietario del valor

    println!("{}", ss);

    let ss: String;
    {
        let s = String::from("hello");
        ss = s.clone();  // s se clona en ss. Ahora ss y s son propietarios de "hello"

        println!("{}", s); // s sigue siendo propietario de "hello"
    } // s sale del ámbito y se libera, pero ss sigue siendo propietario del valor, porque es una copia
    println!("{}", ss); // ss sigue siendo propietario de "hello"

    take_ownership(ss); // ss se mueve a la función take_ownership

    // println!("{}", ss); // Error: ss ya no es propietario de "hello"

    let ss = String::from("hello world");
    let ss = take_ownership_and_return(ss); // ss se mueve a la función take_ownership_and_return y se devuelve
    println!("{}", ss); // ss es propietario de "hello world"

    let (length, ss) = calculate_length(ss); // ss se mueve a la función calculate_length y se devuelve
    println!("Length: {}, ss: {}", length, ss); // ss es propietario de "hello world"

    // El préstamo de referencias permite que una función use un valor sin mover su propiedad. Tenemos que usar el operador
    // de préstamo para indicar que pasamos una referencia a la función
    print_string(&ss); // ss se presta a la función print_string

    // Al terminar la función, ss sigue siendo propietario de "hello world"
    println!("{}", ss);
    
    // Mediante el préstamo mutable, podemos permitir que una función modifique un valor sin mover su propiedad
    let mut ss = String::from("hello");
    // Hay que utilizar el operador de préstamo mutable para indicar que pasamos una referencia mutable a la función
    append_string(&mut ss); // ss se presta como mutable a la función append_string

    // Al terminar la función, ss sigue siendo propietario de "hello world"
    println!("{}", ss);

    // Una referencia mutable solo puede existir una vez en un ámbito
    let r1 = &mut ss;
    //let r2 = &mut ss; // Error: cannot borrow `ss`as mutable more than once at a time

    print_string(r1);
    //print_string(r2);

    // Para evitar el error anterior, podemos crear un nuevo ámbito
    {
        let r2 = &mut ss;
        print_string(r2);

        // Pero en este ámbito no podemos usar r1, o volveremos a tener el error
        // print_string(r1);
    }

    // No podemos tener en un mismo ámbito referencias inmutables y mutables al mismo valor, aunque si que
    // podemos tener más de una referencia inmutable
    let mut ss = String::from("hello again");

    let r1 = &ss;
    let r2 = &ss;
    //let mut r3 = &mut ss; // Cannot borrow `ss` as mutable because it is also borrowed as immutable

    print_string(r1);
    print_string(r2);

    // De nuevo, creando un ámbito podríamos tener la referencia mutable
    {
        let r3 = &mut ss;
        append_string(r3);
    }

    // Pero esto seguiría dando un error, porque hemos mutado ss en el ámbito anterior. Esto es porque r1 existía antes que r3
    // print_string(r1);

    // Pero si que podemos crear una nueva referencia inmutable después de modificarla en un ámbito anterior
    let r4 = &ss;
    print_string(r4);

}

// Si pasamos un valor a una función, se mueve su propietario
fn take_ownership(s: String) {
    println!("take_ownership: {}", s);
}

// Podemos devolver la propiedad otra vez devolviendo el valor
fn take_ownership_and_return(s: String) -> String {
    println!("take_ownership_and_return: {}", s);
    s  // Devolvemos la propiedad de s
}

// Podemos usar tuplas para devolver varios valores. En este caso, devolvemos la longitud de la cadena y la propiedad
fn calculate_length(s: String) -> (usize, String) {
    let length = s.len();
    (length, s)
}

fn print_string(s: &String) {
    println!("Print string: '{}'", s);
}

fn append_string(s: &mut String) {
    s.push_str(" world");
}

// Esto en Rust no se puede hacer, porque la referencia se libera al salir de la función
// Para hacer este tipo de cosas, hay que usar especificadores de tiempo de vida
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }
