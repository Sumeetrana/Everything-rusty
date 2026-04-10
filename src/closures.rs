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

    let values = vec![10, 20, 30, 45, 75, 100];
    let even_vector: Vec<i32> = values.into_iter().filter(|x| x % 2 == 0).collect();

    println!("{:?}", even_vector);
}
