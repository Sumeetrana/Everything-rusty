use std::fs;

fn main() {
    let text = fs::read_to_string("logs.txt");

    match text {
        Ok(value) => {
            println!("{}", value);
        }
        Err(error) => {
            println!("{}", error);
        }
    }
}
