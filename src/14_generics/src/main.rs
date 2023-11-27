
fn main() {
    generics_1();
    generics_2();
}

struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

impl<T: std::fmt::Display + PartialOrd> Point<T> {
    fn cmd_display(&self) {
        if self.x >= self.y {
            println!("the largest member is x = {}", self.x);
        } else {
            println!("the largest member is y = {}", self.y);
        }
    }
}

fn generics_1() {

    let integer = Point::new(5, 10);
    let float = Point::new(1.0, 4.0);

    integer.cmd_display();
    float.cmd_display();

    println!("x = {}, y = {}", integer.x(), integer.y());
    println!("x = {}, y = {}", float.x(), float.y());
}


fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn generics_2() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);

    println!("The largest number is {}", result);
}