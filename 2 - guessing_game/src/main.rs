use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Welcome to, guess the number game!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attemps: u32 = 1;

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You guess it right in {attemps} attemps!");
                break;
            }
        }

        attemps += 1;
    }
}
