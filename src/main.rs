use std::io;
use std::cmp::Ordering;
use rand::Rng;
mod guessing;
use guessing::Guess;

fn read_guess() -> Result<Guess, String> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|_| "Failed to read line".to_string())?;
    
    let num: i32 = input.trim().parse()
        .map_err(|_| "Please enter a valid number.".to_string())?;
    
    Guess::new(num)
        .map_err(|_| "Please enter a number between 1 and 100.".to_string())
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Please input your guess.");

        let guess = match read_guess() {
            Ok(g) => g,
            Err(e) => {
                println!("{}", e);
                continue;
            },
        };
    
    
        println!("You guessed : {}", guess.value());
    
        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

