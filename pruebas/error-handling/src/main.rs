
// Result es un enum definido en la biblioteca estándar de Rust que puede contener dos valores
// - Ok(T) si la operación fue exitosa y el valor de retorno es de tipo T
// - Err(E) si la operación falló y el valor de retorno es de tipo E
// Esta función propaga el error a la función que la llama, en lugar de manejarlo en la función
fn error_if_not_42(x: i32) -> Result<i32, String> {
    if x == 42 {
        Ok(x)
    } else {
        Err(format!("{} is not 42", x))
    }
}

// Esta función utiliza el método abreviado para propagar errores mediante el operador `?`. Este operador
// se puede utilizar en funciones que devuelven Result, y si el Result es Err, la función devolverá Err
fn propagate_error() -> Result<i32, String> {
    let x = error_if_not_42(43)?;
    Ok(x)
}


fn main() {
    // Podemos lanzar un panic con un mensaje personalizado usando la macro panic!
    // panic!("El valor suministrado no es 42");

    // Errores recuperables: se manejan con el enum Error

    // Fun fact: volver a declarar una variable con let se llama shadowing
    let x = 42;
    match error_if_not_42(x) {
        Ok(x) => println!("{} is 42", x),
        Err(e) => println!("{}", e),
    }

    // Como Result es un enumerado, podemos usar match para manejar los dos casos.
    // En este caso no se produce un panic, sino que se imprime el mensaje de error
    let x = 43;
    match error_if_not_42(x) {
        Ok(x) => println!("{} is 42", x),
        Err(e) => println!("{}", e),
    }

    // Los métodos unwrap sirven para extraer el valor de un Result si es Ok, o para manejar un Err.
    // En este caso, el método unrwap_or devuelve el valor de Ok si es Ok, o el valor suministrado en
    // la función en acso contrario
    // Los métodos unwrap disponibles son:
    // - unwrap: devuelve el valor de Ok si es Ok, o lanza un panic con el mensaje suministrado
    // - unwrap_or: devuelve el valor de Ok si es Ok, o el valor suministrado en la función
    // - unwrap_or_else: devuelve el valor de Ok si es Ok, o el valor suministrado en la función
    // - unwrap_err: devuelve el valor de Err si es Err, o lanza un panic con el mensaje suministrado

    let x = error_if_not_42(42)
        .unwrap(); // Esto lanzará un panic si no es Ok
    println!("{}", x);

    let x = error_if_not_42(42)
        .unwrap_or(7);
    println!("{}", x);

    let x = error_if_not_42(43)
        .unwrap_or(7);  // Esto no lanzará un panic si no es Ok, sino que devolverá 0
    println!("{}", x);

    let x = error_if_not_42(43)
        .unwrap_or_default();  // Esto no lanzará un panic si no es Ok, sino que devolverá el valor por defecto
    println!("{}", x);

    let err = error_if_not_42(43)
        .unwrap_err(); // Esto lanza excepción si NO es un error
    println!("{}", err);

    // expect se utiliza para mostrar un mensaje de error personalizado si el Result es Err
    // También lanza un panic si el Result es Err
    let x = error_if_not_42(42)
        .expect("El valor suministrado no es 42");  // Esto lanzará un panic si no es Ok, pero con un mensaje personalizado
    println!("{}", x);

    // unwrap_or_else es similar a unwrap_or, pero en lugar de devolver un valor, se ejecuta una función
    let x = error_if_not_42(43)
        .unwrap_or_else(|e| {
            println!("{}", e);
            3   // El método devuelve el valor que devolverá la función
        });
    println!("{}", x);

    // Propagación de errores: ver función `propagate_error`
    let x = propagate_error()
        .unwrap_or(3);
    println!("{}", x);

}
