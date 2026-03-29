mod math_lib;
use math_lib::maths::*;

fn main() {
    let result = add(3, 2);
    println!("{}", result);

    let result = sub(3, 2);
    println!("{}", result);
}
