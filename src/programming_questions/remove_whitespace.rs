use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed inputs");

    remove_whitespace(&mut input);
}

fn remove_whitespace(input: &mut String) {
    let words = input.split_whitespace().collect::<Vec<&str>>().join("");

    println!("{}", words);

    // let vec: Vec<&str> = words.collect();

    // println!("{}", vec.join(""));
}
