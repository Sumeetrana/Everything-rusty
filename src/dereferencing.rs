fn main() {
    // Dereferencing
    // let x = 10;
    // let y = &x;

    // println!("{}", *y);

    // mutable references
    let mut x = 10;
    let y = &mut x;
    *y = 40;

    println!("{}", x);
}
