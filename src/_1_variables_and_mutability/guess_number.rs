use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

pub fn main() {
    println!("Guess the number from 0 to 100");

    let secret_number: i32 = rand::rng().random_range(1..=100);

    println!("Please input your guess.");

    let mut guess = String::new();

    stdin().read_line(&mut guess).expect("Failed to read line");

    let guess: i32 = guess.trim().parse().expect("guess is not a number");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("{} is too small!", guess),
        Ordering::Greater => println!("{} is too big!", guess),
        Ordering::Equal => println!("You win!"),
    };

    println!(
        "The secret number is: {}, but you guessed: {}",
        secret_number, guess
    );
}
