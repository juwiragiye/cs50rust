use std::io::{self, Write};

fn main() {
    // Get the change owed
    let change = read_change();
    // Get the number of coins
    let coins = get_coins(change);
    // Print the coins.
    println!("{coins}");
}

fn read_change() -> u32 {
    let mut input = String::new();

    loop {
        print!("Change owed: ");
        io::stdout().flush().expect("failed to flush stout");

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(change) = input.trim().parse::<u32>() {
            if change > 0 {
                return change;
            }
        }
    }
}

fn get_coins(amount: u32) -> u32 {
    let mut change = amount;
    let mut coins = 0;

    for coin in [25, 10, 5, 1] {
        coins += change / coin;
        change %= coin;
    }
    return coins;
}
