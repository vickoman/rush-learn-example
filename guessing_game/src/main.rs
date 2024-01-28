use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut count = 0;

    println!("Welcome to the Guessing Game!");
    loop {
        println!("Please input your guess.");


        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                count += 1;
            }
            Ordering::Greater => {
                println!("Too big!");
                count += 1;
            }
            Ordering::Equal => {
                println!("You win! at the  #{count} try");
                break;
            }
        }
    }
}
