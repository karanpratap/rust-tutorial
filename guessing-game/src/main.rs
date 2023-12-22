use std::{io, cmp::Ordering};
use rand::{thread_rng, Rng};

fn main() {
    let random_num = thread_rng().gen_range(1..101);
    println!("Random number: {}", random_num);
    let mut tries = 5;

    loop {
        println!("Enter a number you guessed:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Error");
        // Shadow the value of guess to an integer
        let guess: u32 = match guess.trim_end().trim().parse() {
            Ok(guess) => {
                guess
            },
            Err(e) => {
                println!("Please enter a number. > {}", e);
                continue;
            }
        };

        match guess.cmp(&random_num) {
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater => {
                println!("Too big :(");
            },
            Ordering::Less => {
                println!("Too less :/");
            }
        }

        tries -= 1;
        if tries == 0 {
            println!("You lose - out of turns!");
            break;
        }
    };
    
}
