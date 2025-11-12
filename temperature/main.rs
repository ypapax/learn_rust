use std::io;
fn main() {
    let mut celcius = String::new();
    println!("input your celcius temperature:");
    io::stdin().read_line(&mut celcius).expect("Failed to read line");
    println!("{} Celcius inputed", celcius)
}