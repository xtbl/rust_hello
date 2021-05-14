
use rand::prelude::*;
use std::io;

fn main() {

    // test random number
    assert_eq!((0..11).contains(&get_random_number()), true);

    // test check number guess -> too high, too low, correct
    let number_to_guess = 1;
    let guess = 5;
    assert_eq!(guess_verification(number_to_guess, guess), "number too high");
    assert_eq!(guess_verification(2, 1), "number too low");
    assert_eq!(guess_verification(10, 10), "number is correct");

    println!("Tests passed!");

    let guess_num = get_random_number();

    loop {

      let mut buffer = String::new();
      println!("Enter a number:");
      io::stdin().read_line(&mut buffer);
      println!("buffer is: {}", buffer);
      let user_number: u8 = buffer.trim().parse().unwrap();
      println!("number is: {}", user_number);
      let guess_number_response = guess_verification(guess_num, user_number);
      println!("{}", guess_number_response);

      if guess_number_response == "number is correct" {
        break;
      }
    }
}

fn get_random_number() -> u8 {
    let rand_num = thread_rng().gen_range(1..11);
    rand_num
}

fn guess_verification(number_to_guess: u8, guess: u8) -> String {
    print!("vars {} {} ",number_to_guess, guess);

    match guess {
        guess if number_to_guess < guess => String::from("number too high"),
        guess if number_to_guess > guess => String::from("number too low"),
        guess if number_to_guess == guess => String::from("number is correct"),
        _ => String::from("number is wrong")
    }
}