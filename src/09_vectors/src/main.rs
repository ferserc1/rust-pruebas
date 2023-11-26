
fn main() {
    vectors_1();
    vectors_2();
    vectors_3();
    vectors_4();
    vectors_5();
}

fn vectors_1() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
}

fn vectors_2() {
    let v = vec![1, 2, 3];
    
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    if let Some(third) = third {
        println!("The third element is {}", third);
    }
}

fn vectors_3() {
    let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];

    v.push(6); // ERROR: cannot borrow `v` as mutable because it is also borrowed as immutable
    // no se puede modificar un vector mientras se está accediendo a un elemento, porque el 
    // vector podría cambiar de tamaño y el elemento podría ser movido a otro lugar de la memoria.
    // Para que funcione el código, hemos comentado la referencia a v[0]

   // println!("The first element is: {}", first);
}

fn vectors_4() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }
}

fn vectors_5() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];

    for i in &row {
        match i {
            SpreadsheetCell::Int(value) => println!("Int: {}", value),
            SpreadsheetCell::Float(value) => println!("Float: {}", value),
            SpreadsheetCell::Text(value) => println!("Text: {}", value),
        }
    }
}