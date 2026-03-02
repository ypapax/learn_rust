use std::io;
fn main() {
    println!("Input any url");
    let mut url = String::new();
    io::stdin().read_line(&mut url).expect("Failed to read line");
    println!("Your input {}", url);
}
