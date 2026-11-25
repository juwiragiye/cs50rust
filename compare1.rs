use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    let x = read_int("what is x?: ");
    let y = read_int("what is y?: ");

    match x.cmp(&y) {
        Ordering::Greater => println!("x is greater is than y"),
        Ordering::Less => println!("x is less than y"),
        Ordering::Equal => println!("x equals to y"),
    }
}

fn read_int(prompt: &str) -> i32 {
    print!("{prompt}");

    io::stdout()
        .flush()
        .expect("Faild to flush the stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    input.trim()
        .parse()
        .expect("Please enter a valid number")
}
