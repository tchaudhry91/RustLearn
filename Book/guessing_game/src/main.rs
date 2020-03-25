use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number. This will teach you binary search.");

    // Generates a random number between [1-101)
    let secret = rand::thread_rng().gen_range(1, 101);

    loop {
        let mut guess = String::new();
        println!("Guess:");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read your guess..");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("We're guessing numbers -_-, Try again.");
                continue;
            }
        };

        match guess.cmp(&secret) {
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("Well Done Mate. That's the number.");
                break;
            }
            Ordering::Less => println!("Too Low!"),
        }
    }
}
