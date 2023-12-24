fn main() {
    let line = String::from("world!");
    println!("First word: {}", first_word(&line))
}

fn first_word(line: &String) -> &str {
    for (i, &item) in line.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &line[..i];
        }
    }
    &line[..]
    // &line[..i]
}