use std::io;
fn main() {
    println!("Welcome to guessing game, I have a number on my mind now between 1 and 100, \
    please it, and I will tell you if it is too high or too low or maybe you it is my number.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("{} - this is your guess", input);

    let guess: u32 = input.trim().parse().expect("Please type a number!");
    println!("{} your number", guess)
}