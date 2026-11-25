// A program that compares two numbers
use std::io; // Import I/O library

fn main() {
    println!("What is x?");

    let mut input_x = String::new();
    io::stdin().read_line(&mut input_x).unwrap();
    let x: i32 = input_x.trim().parse().expect("Please enter a number");

    println!("What is y?: ");
    let mut input_y = String::new();
    io::stdin().read_line(&mut input_y).unwrap();
    let y: i32 = input_y.trim().parse().expect("Please enter a number");

    if x > y {
        println!("x is greater than y");
    } else if x < y {
        println!("x is less than y");
    } else {
        println!("x equals to y");
    }
}
