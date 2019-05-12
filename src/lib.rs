use std::cmp::Ordering;
use rand::Rng;

pub struct Guess {
    pub input: u32,
    pub min: u32,
    pub max: u32,
}

pub fn guess(guess: Guess) -> String {
    check_with_custom_random(&get_random_number, guess)
}

fn get_random_number(min: u32, max: u32) -> u32 {
    rand::thread_rng().gen_range(min, max + 1)
}

fn check_with_custom_random(random: &Fn(u32, u32) -> u32, guess: Guess) -> String {
    let secret_number = random(guess.min, guess.max);

    match guess.input.cmp(&secret_number) {
        Ordering::Less => String::from("Too small!"),
        Ordering::Greater => String::from("Too big!"),
        Ordering::Equal => String::from("You win!"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{check_with_custom_random, get_random_number, Guess};

    fn build_guess(input: u32) -> Guess {
        Guess { input, min: 1, max: 100 }
    }

    fn random_mock(_min: u32, _max: u32) -> u32 { 4 }

    #[test]
    fn guess_right() {
        let guess = build_guess(4);
        let result = check_with_custom_random(&random_mock, guess);
        assert_eq!(String::from("You win!"), result);
    }

    #[test]
    fn guess_too_small() {
        let guess = build_guess(2);
        let result = check_with_custom_random(&random_mock, guess);
        assert_eq!(String::from("Too small!"), result);
    }

    #[test]
    fn guess_too_big() {
        let guess = build_guess(8);
        let result = check_with_custom_random(&random_mock, guess);
        assert_eq!(String::from("Too big!"), result);
    }

    #[test]
    fn generate_random_number() {
        assert_eq!(1, get_random_number(1, 1));
        assert_eq!(3, get_random_number(3, 3));
        assert_eq!(11, get_random_number(11, 11));
    }
}
