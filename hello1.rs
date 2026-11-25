// A program that says her to someone with their name

use std::io; // 
use std::io::Write;

fn main() {
    print!("What is your name?: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .unwrap();
    println!("hello, {}", name.trim());
}

