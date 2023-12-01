fn benchmark_for(words: &Vec<&str>) {
    let mut count = 0;
    for word in words {
        if word.contains("to") {
            count += 1;
        }
    }
    println!("        \"count\": {},", count);
}

fn benchmark_iterator<'a>(words: impl Iterator<Item = &'a str>) {
    let mut count = 0;
    words.for_each(|word| {
        if word.contains("to") {
            count += 1;
        }
    });
    println!("        \"count\": {},", count);
}

fn main() {
    let contents = std::fs::read_to_string("text.txt")
        .expect("Something went wrong reading the file");

    let contents = contents.split_whitespace().collect();
    println!("{{");

    println!("    \"forLoop\": {{");
    let now = std::time::Instant::now();
    benchmark_for(&contents);
    let elapsed = now.elapsed();
    let elapsed = elapsed.subsec_nanos() as u64;
    println!("        \"elapsed\": {:?},", elapsed);
    println!("        \"units\": \"ns\"");
    println!("    }},");

    println!("    \"iterator\": {{");
    let now = std::time::Instant::now();
    benchmark_iterator(contents.into_iter());
    let elapsed = now.elapsed();
    let elapsed = elapsed.as_secs() * 1_000_000_000 + elapsed.subsec_nanos() as u64;
    println!("        \"elapsed\": {:?},", elapsed);
    println!("        \"units\": \"ns\"");
    println!("    }}");

    println!("}}");
}
