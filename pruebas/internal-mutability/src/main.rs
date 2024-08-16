use core::time;
use std::{cell::RefCell, rc::Rc, sync::{Arc, Mutex, RwLock}};

struct MyData {
    value: i32
}

fn main() {
    println!("Single threaded example of internal mutability:");
    let data = Rc::new(RefCell::new(MyData { value: 0 }));

    let first_owner = data.clone();
    let second_owner = data;

    first_owner.borrow_mut().value += 1;

    {
        // Podemos acceder al valor de RefCell y asignarlo a una variable,
        // pero siempre dentro de un bloque distinto.
        // Si hacemos esto fuera de un bloque, estaremos incumpliendo las
        // reglas de referencia y préstamo y fallará en tiempo de ejecución
        let mut cosa = first_owner.borrow_mut();
        cosa.value += 1;
    }
   
    println!("{}", second_owner.borrow().value);
    
    first_owner.borrow_mut().value += 2;

    println!("{}", second_owner.borrow().value);
    
    first_owner.borrow_mut().value += 3;

    println!("{}", second_owner.borrow().value);

    // Para escenarios multi-hilo cambiamos Rc por Arc y Mutex o RwLock por RefCell
    // Con RwLock varios hilos pueden acceder a la vez para lectura, pero solamente
    // uno puede acceder para escritura
    println!("Multi threaded example of internal mutability using RwLock:");
    let data = Arc::new(RwLock::new(MyData { value: 0 }));

    let first_owner = data.clone();
    let second_owner = data;

    println!("{}", second_owner.read().unwrap().value);

    std::thread::spawn(move || {
        first_owner.write().unwrap().value += 1;
    });

    std::thread::sleep(time::Duration::from_secs(1));

    println!("{}", second_owner.read().unwrap().value);


    // Con Mutex solo un hilo puede acceder a la vez, ya sea para lectura o escritura
    println!("Multi threaded example of internal mutability using Mutex:");
    let data = Arc::new(Mutex::new(MyData { value: 0 }));

    let first_owner = data.clone();
    let second_owner = data;

    println!("{}", second_owner.lock().unwrap().value);

    std::thread::spawn(move || {
        first_owner.lock().unwrap().value += 1;
    });

    std::thread::sleep(time::Duration::from_secs(1));

    println!("{}", second_owner.lock().unwrap().value);

}
