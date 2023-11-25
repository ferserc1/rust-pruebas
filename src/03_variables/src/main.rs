fn mutability() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn shadowing() {
    let x = 5;
    println!("The value of x is: {x}");

    // Redefinir el valor de x inmutable
    let x = x + 1;
    println!("The value of x is: {x}");

    // Redefinir el tipo
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    // Redefinir la mutabilidad
    let mut spaces = "   ";
    println!("{}", spaces); // "   "
    spaces = "Another string";
    println!("{}", spaces); // "Another string"
}

fn overflow() {
    let mut x: u8 = 255;
    println!("The value of x is: {x}");
    // x = x + 1;
    x = u8::saturating_add(x, 1);
    println!("The value of x is: {x}");

    x = u8::wrapping_add(x, 1);
    println!("The value of x is: {x}");

    x = 255;
    let overflow_x = u8::overflowing_add(x, 1);
    println!("The value of x is: {x}", x = overflow_x.0);
    if overflow_x.1 {
        println!("Overflow!");
    }

    let x = 255;
    match u8::checked_add(x, 1) {
        Some(value) => println!("The value of x is: {x}", x = value),
        None => println!("Overflow!"),
    }
}

fn numeric_operators() {
    let sum = 5 + 10;
    println!("The value of sum is: {sum}");

    let difference = 95.5 - 4.3;
    println!("The value of difference is: {difference}");

    let product = 4 * 30;
    println!("The value of product is: {product}");

    let quotient = 56.7 / 32.2;
    println!("The value of quotient is: {quotient}");
    let truncated = -5 / 3;
    println!("The value of truncated is: {truncated}");

    let remainder = 43 % 5;
    println!("The value of remainder is: {remainder}");
}

fn characters() {
    let c = 'z';
    let z: char = 'ℤ'; // Con anotación explícita
    let heart_eyed_cat = '😻';
    println!("The value of c is: {c}");
    println!("The value of z is: {z}");
    println!("The value of heart_eyed_cat is: {heart_eyed_cat}");
}

fn tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    let (x, y, z) = tup;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");

    // Imprimir valores individuales de tupla
    println!("The value of tup.0 is: {}", tup.0);
    println!("The value of tup.1 is: {tup_1}", tup_1 = tup.1);
    println!("The value of tup.2 is: {tup_2}", tup_2 = tup.2);
}

fn arrays() {
    // Definiciones con tipo implícito
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", 
        "July", "August", "September", "October", "November", "December"];

    // Acceder a elementos
    let first = months[0];
    let second = months[1];
    println!("The value of a[0]: {}", a[0]);
    println!("The value of first month is: {first}");
    println!("The value of second month is: {second}");

    // Definir el tamaño y tipo de los elementos
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of a[1] is: {}", a[1]);

    // Definir un array de 5 elementos con el valor 3
    let a = [3; 5];
    println!("The value of a[1] is: {}", a[1]);
}

fn array_out_of_bounds() {
    let a = [1, 2, 3, 4, 5];

    println!("Enter an array index between 0 and 4: ");
    let mut index = String::new();
    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("The value of the element at index {} is: {}", index, element);
}

fn main() {
    mutability();    
    shadowing();
    overflow();
    numeric_operators();
    characters();
    tuple();
    arrays();
    array_out_of_bounds();
}
