use std::io;

use rand::Rng;
fn main() {
    println!("Welcome to guessing game, I have a number on my mind now between 1 and 100, \
    please it, and I will tell you if it is too high or too low or maybe you it is my number.");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;
    loop {
        attempts+=1;
        println!("Please input your guess.");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        println!("{} - this is your guess", input);

        let guess: u32 = input.trim().parse().expect("Please type a number!");
        println!("{} your number", guess);
        if guess == secret_number {
            println!("You win! Attempts amount: {}", attempts);
            println!("The secret number is: {}", secret_number);
            break;
        } else if guess < secret_number {
            println!("Too low!");
        } else {
            println!("Too high!");
        }
    }


}