use std::io::{self, Write};

fn main() {
    // Get the number of rows
    let height = read_height();

    //Print the pyramid
    print_pyramid(height);
}

fn read_height() -> u32 {
    let mut input = String::new(); // Alocate the memory for the input
    
    // Get a number that is positive and greater than zero
    loop {
        print!("Height: ");
        io::stdout().flush().expect("Failed to flush stdout");

        input.clear(); // Clear any text in the previous buffer
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(height) = input.trim().parse::<u32>() {
            if height > 0 {
                return height;
            }
        }
    }
}

fn print_pyramid(h: u32) {
    let height = h as usize;
    
    for i in 1..=height {
        let spaces = " ".repeat(height-i);
        let hashes = "#".repeat(i);
        println!("{}{} {}", spaces, hashes, hashes);
    }
}
