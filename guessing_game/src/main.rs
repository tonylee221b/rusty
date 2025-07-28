use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // loop is like 'while' loop
    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // let mut <<- the variable is mutable

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // .expect
                                            // on "Err": throw with provided String,
                                            // on "Ok": returns Ok result value

        // Rust allows shadowing
        //let guess: u32 = guess.trim().parse().expect("Please type a number!"); // type convertion
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // match is similiar to case.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // exits loop
            }
        }
    }
}
