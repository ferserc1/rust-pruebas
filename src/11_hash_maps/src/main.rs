use std::collections::HashMap;

fn main() {
    hash_map_1();
    hash_map_2();
    hash_map_3();
}

fn hash_map_1() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Puntuación del equipo {team_name}: {score}");

    for (team, score) in &scores {
        println!("Puntuación del equipo {team}: {score}");
    }
}

fn hash_map_2() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);
}

fn hash_map_3() {
    let text = "hola mundo maravilloso mundo";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    for (word, count) in &map {
        let times = if *count == 1 { "vez" } else { "veces" };
        println!("La palabra {word} aparece {count} {times}");
    }
}