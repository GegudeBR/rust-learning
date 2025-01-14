use std::io::{self, Write};
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let target:i8 = rng.gen_range(1..=100);

    println!("You have to guess a number from 1 to 100.");

    let mut correct = false;
    let mut guesses = 0;
    while !correct {
        let guess = loop {
            print!("Enter your guess: ");
            let _ = io::stdout().flush();
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            if let Ok(val) = input_line.trim().parse::<i8>() {
                if val >= 0 && val <= 100 {
                    break val;
                } 
            } else {
                println!("The guess needs to be between 0 and 100");
            }
        };
       
        guesses += 1;s
        let difference = i8::abs(target - guess);
        match difference {
            x if x == 0 => (),
            x if x < 10 => println!("Close..."),
            x if x < 25 => println!("Almost there."),
            _ => (),
        };

        match guess {
            x if x > target => println!("Too high! Try again."),
            x if x < target => println!("Too low! Try again."),
            x if x == target => {
                correct = true;
                println!("Correct! The secret number is {}!", target);
                println!("You found the number in {} guesses.", guesses);
            },
            _ => panic!(),
       }
       
    }
}
