use std::{cmp::Ordering, io::{self, Write}};

fn main() {
    println!("Guess a number between 1 and 100");
    
    let secret_number: u32 = rand::random_range(1..=100);
    
    loop {
        print!("Guess: ");
        io::stdout().flush().expect("failed to flush stdout");

        let mut guess = String::new();

        guess.clear();
        io::stdin()
            .read_line(&mut guess)
            .unwrap();

        // Handle invalid inputs
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        // Compare our secret number to user input
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("too high"),
            Ordering::Less => println!("too low"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}
