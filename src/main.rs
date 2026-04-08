fn main() {
    // let add_numbers = |x: i32| x + 1;
    // println!("{}", add_numbers(3));

    let mut counter = 0;

    let mut increment = || {
        counter = counter + 1;
        println!("{}", counter);
    };

    increment();
    increment();
    increment();
    increment();
    increment();
}
