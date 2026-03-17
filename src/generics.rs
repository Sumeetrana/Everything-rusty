use num_traits::Float;

fn solve<T: Float>(a: T, b: T) -> T {
    (a.powi(2) + b.powi(2)).sqrt()
}

fn main() {
    let a: f32 = 3.0;
    let b: f32 = 4.0;
    println!("{}", solve(a, b));
}
