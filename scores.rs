use std::io::{self, Write};
const N: u32 = 3;

fn main() {
    let mut total = 0.0;
    let mut input = String::new();

    for _i in 0..N {
        print!("Score: ");
        io::stdout().flush().expect("Failed to flush io");

        input.clear(); // clear any input before
        io::stdin()
            .read_line(&mut input)
            .unwrap();
        if let Ok(score) = input.trim().parse::<u32>() {
            if score > 0 {
                total += score as f64;
            }
        }
    }
    let average = total / (N as f64);
    println!("Average: {average:.2}");
}