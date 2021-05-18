use std::io;
use rand::prelude::*;

fn main() {
    let secret_number = thread_rng().gen_range(1..11);
    println!("I'm thinking of a number between 1 and 100...");
    println!("Guess the number:");
    loop {
        let mut guess = String::new();
        match io::stdin().read_line(&mut guess) {
          Ok(n) => {
              println!("{} read line", n);
              println!("{}", guess);
          }
          Err(error) => println!("error: {}", error),
      }
        let mut valid_result;
        let mut guess_result = 0;
        match guess.trim().parse::<u32>() {
            Ok(result) => {
              guess_result = result;
              valid_result = true;
            },
            Err(error) => {
              println!("error {}", error);
              valid_result = false;
            }
        };

        if valid_result {
            if guess_result > secret_number {
                println!("\n{} is too high! Guess lower:", guess);
            } else if guess_result < secret_number {
                println!("\n{} is too low! Guess higher:", guess);
            } else {
                println!("\nYou got it! The secret number was {}.", secret_number);
                break;
            }
        } else {
            println!("Result wasn't valid");
            break;
        }
    }
}
