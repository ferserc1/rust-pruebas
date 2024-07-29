// Un slice es una secuencia congiunta de elementos de una colección. Es un tipo de referencia, por lo que no tiene posesión

fn main() {
    let s = String::from("hello world");

    let hello = &s[..5];   // Si el primer índice es cero, se puede omitir
    let world = &s[6..11];
    let hello_world = &s[..];    // Slice de toda la cadena

    println!("{} {}", hello, world);
    println!("{}", hello_world);

    let first_word = first_word(&s);
    println!("The first word is '{}'", first_word);

    // Podemos usar slices como literales de cadenas de texto. En este caso, el valor de la cadena se almacena en el propio código de la aplicación,
    // y el slice se refiere a ese valor
    let s = "This is a literal string";
    println!("{}", s);
}

fn first_word(word: &String) -> &str { // El tipo de un slice a un string es &str
    let bytes = word.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &word[..i];
        }
    }

    &word[..]   // Si no hay espacios, se devuelve la palabra completa
}