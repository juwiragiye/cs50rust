use std::io::{self, Write};

fn main() {
    // Get the height of the pyramid
    let height = read_height();
    // Print the pyramid using #
    print_pyramid(height);
}

fn read_height() -> u32 {
    loop {
        // prompt user for height
        print!("Height: ");
        io::stdout().flush().expect("Failed to flush the stout");

        // return the number if it's a positive integere and greater than zero
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(num) = input.trim().parse::<u32>() {
            if num > 0 {
                return num;
            }
        }
    }
}

fn print_pyramid(height: u32) {
    let rows = height as usize;
    for i in 1..=rows {
        let hashes = "#".repeat(i);
        let spaces = " ".repeat(rows - i);
        println!("{}{}", spaces, hashes);
    }
}
