use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter your input: ");

    io::stdin().read_line(&mut input).expect("Failed inputs");

    reverse_string(&mut input);
}

fn reverse_string(input: &mut String) {
    // let mut result = input.trim().split("").collect::<Vec<&str>>();

    // result.reverse();

    // println!("{:?}", result.join(""));

    let result: String = input.trim().chars().rev().collect();
    println!("{:?}", result);
}
