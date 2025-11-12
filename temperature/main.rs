use std::io;
fn main() {
    let mut input = String::new();
    println!("input your celcius temperature:");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("{} Celcius inputed", input);
    let celc: f32 = input.trim().parse().expect("Failed to parse input");
    println!("{} Celcius parsed", celc);
    let fahr = celc * 1.8 + 32.0;
    println!("{} Fahrenheit", fahr);
}