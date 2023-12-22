#![allow(unused)]

use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let mut name = String::new();
    const GREETING: &str = "Ohayo gozaimashita!";
    io::stdin().read_line(&mut name).expect("Please enter a name");
    let random_num = rand::thread_rng().gen_range(1..101);
    let store = "47";
    let mut store: u32 = store.parse()
            .expect("NaN");
    println!("Hello, {}! {} {} years old person {}.", name.trim_end(), GREETING, random_num, store);
  
    let arr = [1, 2, 3, 4, 5];

    match arr[0].cmp(&random_num) {
        Ordering::Less => println!("Hello!"),
	_ => println!("hello")
    };

    for item in arr {
        println!("{}", type_of(item));
        println!("{}", item);
    }

    let mut my_tuple = (47, "hello", 50_000.00);

    println!("{}", type_of(my_tuple.1));
    my_tuple.1 = "what";
    my_tuple.1;
    println!("{}", my_tuple.1);

    let tup1 = (100_000, "Let's get rusty");
    let (subs, channel) = tup1;
    println!("{:?}", tup1);


    // loop {
        
}
