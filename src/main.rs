use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line");

    let guess: i32 = guess.trim().parse().expect("Please type a number");

    match guess.cmp(&secret_number) {
        Ordering::Equal => println!("You win!"),
        Ordering::Less => println!("Your guessed value is small"),
        Ordering::Greater => println!("Your guessed value is large"),
    }

    println!("The secret number is: {secret_number}");
    println!("You guessed: {guess}");
}
