use std::io::{self, Write};

fn main() {
    println!("Guess a number between 1 and 100");
    let mut guess: u32 = 0;
    let number: u32 = rand::random_range(1..=100); // Get a number between 
    
    while guess != number {
        let mut input = String::new();

        print!("Enter a guess: ");
        io::stdout().flush().expect("failed to flush stdout");
        
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(userguess) = input.trim().parse::<u32>() {
            guess = userguess;
            if number < guess {
                println!("Your guess is too high");
            }
            else if number > guess {
                println!("Your guess is too low");
            }
            else {
                println!("Congratulations you guessed the right number {number}")
            }
        }
    }
    
}
