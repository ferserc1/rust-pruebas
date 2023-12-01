
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    println!("{:?}", v1_iter.next());   // Some(1)
    println!("{:?}", v1_iter.next());   // Some(2)
    println!("{:?}", v1_iter.next());   // Some(3)
    println!("{:?}", v1_iter.next());   // None
}

fn iterator_demonstration_2() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let sum: i32 = v1_iter.sum();

    println!("{}", sum);    // 6

    //v1_iter.next(); // error[E0382]: use of moved value: `v1_iter`
}

fn iterator_demonstration_3() {
    let v1 = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("{:?}", v2);   // [2, 3, 4]
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn iterator_demonstration_4() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") }
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    println!("{:#?}", in_my_size);   // [Shoe { size: 10, style: "sneaker" }, Shoe { size: 10, style: "boot" }]
}

fn main() {
    iterator_demonstration();
    iterator_demonstration_2();
    iterator_demonstration_3();
    iterator_demonstration_4();
}
