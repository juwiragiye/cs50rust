use std::io::{self, Write};

fn main() {
    let n = read_positive_int();
    meow(n);
}

fn read_positive_int() -> u32 {
    loop {
        print!("Number: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(n) = input.trim().parse::<u32>() {
            if n > 0 {
                return n;
            }
        }

        println!("Please enter a positive number greater than 0");
    }
}
fn meow(number: u32) {
    for _i in 0..number {
        println!("meow");
    }
}
