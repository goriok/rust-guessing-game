use rust_guessing_game::check;

fn main() {
    use std::io;

    println!("Guess the number!");

    println!("Please input your guess.");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = input.trim().parse()
        .expect("Please type a number!");

    println!("You guessed: {}", check(input));
}