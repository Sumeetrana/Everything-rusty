use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter your input: ");

    io::stdin().read_line(&mut input).expect("Failed inputs");

    check_palindrome(&mut input);
}

fn check_palindrome(input: &mut String) {
    let reversed_string: String = input.trim().chars().rev().collect();

    if reversed_string == input.trim().to_string() {
        println!("True");
    } else {
        println!("False");
    }
}
