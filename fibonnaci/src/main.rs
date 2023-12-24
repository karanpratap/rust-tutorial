use std::io;

fn fib(n: &u32) {
    let mut f = (0, 1);
    for _i in 1..=*n {
        println!("{}", f.0);
        let temp = f.0;
        f.0 = f.1;
        f.1 = temp + f.0;
    }
}

fn main() {
    println!("Enter the number of fibonnaci numbers to be printed:");
    
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Reading the number failed");
    let num: u32 = match num.trim_end().trim().parse() {
        Ok(num) => {
            num
        }
        Err(_) => {
            println!("Wrong number entered");
            0
        }
    };

    fib(&num);
    
}
