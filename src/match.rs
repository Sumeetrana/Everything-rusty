fn main() {
    // let x = 5;

    // match x {
    //     1 => println!("1"),
    //     5 => println!("5"),
    //     _ => println!("Other"),
    // }

    let x: i32 = 7;

    match x {
        n if is_even(n) => println!("Even"),
        n if !is_even(n) => println!("Odd"),
        _ => println!("Other"),
    }
}

fn is_even(n: i32) -> bool {
    n % 2 == 0
}
