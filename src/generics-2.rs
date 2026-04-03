use std::fmt::Display;

fn print_value<T: Display>(value: T) {
    println!("Value is {}", value);
}

fn main() {
    let num = 5;
    print_value(num);

    let decimal = 5.1;
    print_value(decimal);

    let status = true;
    print_value(status);
}
