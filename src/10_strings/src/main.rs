fn main() {
    strings_1();
    strings_2();
    strings_3();
}

fn strings_1() {
    // Crear string vacío y agregarle contenido
    let mut s = String::new();
    s.push_str("hello");
    println!("{}", s);

    // Crear string con contenido
    let data = "initial contents";
    let s = data.to_string();
    println!("{}", s);


    // Crear string con contenido mediante la función from
    let s = String::from("initial contents");
    println!("{}", s);
}

fn strings_2() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // s1 no se puede usar después de esta línea porque se ha movido a s3
    let s3 = s1 + &s2;

    // println!("s1: {}", s1); // ERROR
    println!("s2: {}", s2);
    println!("s3: {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}"); // tic-tac-toe
    println!("{}", s);
}

fn strings_3() {
    let cannon = "cañón";
    // Esto es peligroso: si en lugar de usar el rango 2..4 usamos 2..3, el programa falla
    // porque el caracter ñ ocupa dos bytes
    let s = &cannon[2..4];
    println!("{}", s);

    for c in cannon.chars() {
        println!("{}", c);
    }
}