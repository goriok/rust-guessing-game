use rust_guessing_game::{guess, Guess};

fn main() {
    use std::io;
    let min = 1;
    let max = 100;

    println!("Guess the number! Range: ({}-{})", min, max);
    println!("Please input your guess.");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = input.trim().parse()
        .expect("Please type a number!");

    let result = guess(Guess { input, min, max });

    println!("You guessed: {}, {}", input, result);
}