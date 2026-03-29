use std::io;

fn main() {
    let mut input = String::new();
    println!("Please input your name:");
    io::stdin().read_line(&mut input).expect("Input failed");
    println!("User input: {}", input);
}
