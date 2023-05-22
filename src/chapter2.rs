use std::{cmp::Ordering, io};
use rand::Rng;


pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}


pub fn guessing_game() {
    println!("Guess the number!");

    let latent_number = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess.");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: Guess = match guess.trim().parse() {
                Ok(num) => Guess::new(num),
                Err(_) => continue,
        };

        println!("You guessed: {}", guess.value());

        match guess.value().cmp(&latent_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Just right!");
                break;
            }
        }
    }
}