use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 100);
    //println!("Sercet number is {}", &secret_number);
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };
        if guess < 1 || guess > 100 {
            println!("The secret number will be betwwen 1 and 100.");
            continue;
        }
        println!("Your guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("small"),
            Ordering::Greater => println!("big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}


pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must between 1 and 100, got {}.", value);
        }
        Guess{
            value,
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
