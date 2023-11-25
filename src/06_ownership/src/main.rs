
fn string_test() {
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1);  // ERROR: value borrowed here after move
    println!("{}", s2);

    let s3 = s2.clone();
    println!("s2: {}, s3: {}", s2, s3);
}

fn functions_ownership() {
    let s = String::from("hello");

    takes_ownership(s);
    // println!(s); ERROR: value borrowed here after move

    let s2 = String::from("hello");
    let s2 = takes_and_gives_back(s2);
    println!("{}", s2);

    let s3 = String::from("hello");
    let (s3, len) = calculate_length(s3);
    println!("The length of '{}' is {}.", s3, len);

    let s4 = String::from("hello");
    let len = calculate_length_ref(&s4);
    println!("The length of '{}' is {}.", s4, len);

    let mut s5 = String::from("hello");
    change(&mut s5);
    println!("{}", s5);

    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s; ERROR! cannot borrow `s` as mutable more than once at a time

    println!("{}", r1);
}

fn mutable_references() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);
    // Si ya no usamos r1 y r2 después de este punto, podemos crear una nueva referencia mutable sin problemas.

    let r3 = &mut s;
    println!("{}", r3);
}

fn slices() {
    let mut s = String::from("hello world");

    let pos = first_word_last_index(&s);

    println!("The first word end position is: {}", pos);

    s.clear();
    // la posición de la primera palabra sigue siendo la misma, pero la cadena está vacía

    let mut s = String::from("hello world");

    // Slices de una cadena
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("hello: {}, world: {}", hello, world);

    let first_word_in_string = first_word(&s);
    println!("The first word is: {}", first_word_in_string);
    s.clear();

    let s: &str = "Hello, world!";
    println!("{}", s);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[..3];
    let slice2: &[i32] = &a[3..];
    println!("{:?}", a);
    println!("{:?}", slice);
    println!("{:?}", slice2);
}

fn main() {
    println!("Hello, world!");
    string_test();
    functions_ownership();
    mutable_references();
    slices();
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn takes_and_gives_back(a_string: String) -> String {
    println!("{}", a_string);
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word_last_index(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() { // iter() devuelve cada elemento de la colección
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]   
}
