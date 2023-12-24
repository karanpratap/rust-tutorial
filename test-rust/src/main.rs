struct User {
    username: String,
    password: String
}

fn main() {
    let words = "Hello world";
    let _first = first_word(words);

    let user1 = User {
        username: String::from("kps"),
        password: String::from("pass")
    };

    // Since String does not implement the copy trait, it is moved
    // when we do this
    // let mut user2 = User {
    //     ..user1
    // };

    println!("User is {} with password {}", user1.username, user1.password);

    println!("Words: {}", words);
    println!("Words: {}", _first);
}

fn first_word(line: &str) -> &str {
    for (i, &item) in line.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &line[..i];
        }
    }
    &line[..]
}