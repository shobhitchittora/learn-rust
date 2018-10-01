extern crate rand;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("----- Guessing Game!! -------");

    println!("Make a guess now");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess!! ⁉️");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Input!! Try again!!");
                continue;
            }
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("You win!!");
                break;
            }
        }
    }
}
