use std::io;

fn farenheit_to_celsius(temp: &f32) -> f32 {
    (temp - 32.0) * 5.0 / 9.0
}

fn celsius_to_farenheit(temp: &f32) -> f32 {
    temp * 9.0 / 5.0 + 32.0
}

fn main() {
    println!("1. Farenheit to celsius");
    println!("2. Celsius to farenheit");
    println!("3. Exit");

    loop {
        let mut temp = String::new();
        let mut choice = String::new();
        println!("Choose your functionality (1/2/3):");
        io::stdin().read_line(&mut choice).expect("Failed to read choice");

        match choice.trim_end().trim() {
            "1" => {
                println!("Enter the temperature in farenheit:");
                io::stdin().read_line(&mut temp).expect("Failed to read temp");
                let temp: f32 = match temp.trim_end().parse() {
                    Ok(temp) => temp,
                    Err(_) => {
                        println!("Enter valid temperature...");
                        continue;
                    }
                };
                
                println!("Temperature in celsius: {} C", farenheit_to_celsius(&temp));
            },
            "2" => {
                println!("Enter the temperature in celsius:");
                io::stdin().read_line(&mut temp).expect("Failed to read temp");
                let temp: f32 = match temp.trim_end().trim().parse() {
                    Ok(temp) => temp,
                    Err(_) => {
                        println!("Enter valid temperature...");
                        continue;
                    }
                };
                
                println!("Temperature in farenheit: {} F", celsius_to_farenheit(&temp));
            },
            "3" => {
                break;
            },
            _ => {
                println!("Wrong choice!");
                continue;
            }
        }
    }
}
