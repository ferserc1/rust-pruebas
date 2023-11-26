use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Panic explícito
    // panic!("crash and burn");

    // Panic provocado por un índice fuera de rango
    // let v = vec![1, 2, 3];
    // v[99];

    error_handling_1();
    error_handling_2();
    error_handling_3();
    error_handling_4();

    let text = "Hola\nMundo";
    println!("Último carácter de la primera línea: {:?}", last_char_of_first_line(text));

    let greeting_file = std::fs::File::open("does_not_exists.txt");
    println!("Fichero: {:?}", greeting_file);
    Ok(())
}

fn error_handling_1() {
    use std::fs::File;
    use std::io::ErrorKind;

    let file_name = "hello.txt";
    let greeting_file_result = File::open(file_name);
    match greeting_file_result {
        Ok(file) => { 
            println!("El archivo existe");
            println!("{:?}", file);
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(file_name) {
                Ok(fc) => {
                    println!("El archivo ha sido creado");
                    println!("{:?}", fc);
                },
                Err(e) => {
                    panic!("No se pudo crear el archivo: {:?}", e);
                }
            },
            other_error => {
                panic!("Problema al abrir el archivo: {:?}", other_error);
            }
        }
    };
}

fn error_handling_2() {
    use std::fs::File;
    use std::io::ErrorKind;

    let file_name = "file_2.txt";
    let greeting_file = File::open(file_name).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(file_name).unwrap_or_else(|error| {
                panic!("No se pudo crear el archivo: {:?}", error);
            })
        } else {
            panic!("Problema al abrir el archivo: {:?}", error);
        }
    });

    println!("Fichero: {:?}", greeting_file);
}

fn error_handling_3() {
    use std::fs::File;
    
    let file_name = "hello.txt";
    let greeting_file = File::open(file_name)
        .expect("El fichero `hello.txt` debería existir");
    println!("Fichero: {:?}", greeting_file);
}

fn error_handling_4() {
    let user_name = read_username_from_file("username.txt")
        .expect("No se ha podido leer el nombre de usuario");
    println!("Nombre de usuario: {}", user_name);

    let not_existing_user = match read_username_from_file_shortcut("not_existing_user.txt") {
        Ok(user_name) => user_name,
        Err(_error) => {
            println!("No se encuentra el fichero de usuario. Utilizando usuario por defecto");
            String::from("default_user")
        }
    };
    println!("Nombre de usuario: {}", not_existing_user);
}

fn read_username_from_file(file_name: &str) -> Result<String, std::io::Error> {
    use std::fs::File;
    use std::io::Read;

    let file_result = File::open(file_name);

    let mut username_file = match file_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut s = String::new();

    match username_file.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(error) => Err(error),
    }
}

fn read_username_from_file_shortcut(file_name: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(file_name)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
