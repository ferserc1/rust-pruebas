fn main() {
    println!("Hello, world!");
    another_function();
    function_with_parameter(5);
    add(3,5);
    block_expression();

    if is_greater_than_100(101) {
        println!("Greater than 100");
    } else {
        println!("Less than 100");
    }

    if is_greater_than_100(50) {
        println!("Greater than 100");
    } else {
        println!("Less than 100");
    }
}

fn another_function() {
    println!("Another function.");
}

fn function_with_parameter(x: i32) {
    println!("The value of x is: {x}");
}

fn add(x: i32, y: i32) {
    println!("{x} + {y} = {}", x + y);
}

fn block_expression() {
    let y = {
        let x = 3;
        x + 1
    };
    
    println!("The value of y is: {y}");
}

fn is_greater_than_100(x: i32) -> bool {
    if x > 100 {
        true
    } else {
        false
    }
}