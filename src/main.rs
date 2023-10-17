use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Guess the number!");
    let secret_num:u32 = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut guess = String::new();

        println!("Enter your guess.");
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Greater => println!("{}","Too big!".red()),
            Ordering::Equal => {
                println!("{}","You win! Congratulations.".green());
                break;
            },
        }
    }
}
