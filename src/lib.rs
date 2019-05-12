use std::cmp::Ordering;
use rand::Rng;

pub fn check(guess: u32) -> String {
    check_with_custom_random(&get_random_number, guess)
}

fn get_random_number() -> u32 {
    rand::thread_rng().gen_range(1, 101)
}

fn check_with_custom_random(random: &Fn() -> u32, guess: u32) -> String {
    let secret_number = random();

    match guess.cmp(&secret_number) {
        Ordering::Less => String::from("Too small!"),
        Ordering::Greater => String::from("Too big!"),
        Ordering::Equal => String::from("You win!"),
    }
}

#[cfg(test)]
mod tests {
    use crate::check_with_custom_random;

    fn random_mock() -> u32 {
        4
    }

    #[test]
    fn guess_right() {
        assert_eq!(check_with_custom_random(&random_mock, 4), String::from("You win!"));
    }

    #[test]
    fn guess_too_small() {
        assert_eq!(check_with_custom_random(&random_mock, 2), String::from("Too small!"));
    }

    #[test]
    fn guess_too_big() {
        assert_eq!(check_with_custom_random(&random_mock, 8), String::from("Too big!"));
    }
}
