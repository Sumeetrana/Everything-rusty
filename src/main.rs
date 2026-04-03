#[derive(Debug)]
struct Rectangle {
    length: u16,
    breadth: u16,
}

impl Rectangle {
    fn area(self) -> u16 {
        self.breadth * self.length
    }
}

fn main() {
    let rec = Rectangle {
        length: 20,
        breadth: 20,
    };

    println!("Area: {}", rec.area());
}
