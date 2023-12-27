// Take a word and convert it to pig-latin
fn convert_to_pig_latin(word: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let first_char = word.chars().next().unwrap();
    let mut res = String::new();
    
    for item in vowels {
        if item == first_char {
            println!("Found {}", item);
            res.push_str(word);
            res.push_str("-hay");
            return res;
        }
    }

    res.push_str(&word[1..]);
    res.push('-');
    res.push(first_char);
    res.push_str("ay");
    return res;

}

fn main() {
    let line = "This is a very complicated word combination evangelion";
    let mut pig_latin_line = String::new();

    for word in line.split_whitespace() {
        pig_latin_line.push_str(&convert_to_pig_latin(word)[..]);
        pig_latin_line.push_str(" ");
    }

    println!("Original: {}", line);
    println!("Pig latined: {}", pig_latin_line);

}
