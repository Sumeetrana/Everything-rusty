#[derive(Debug)]
struct Rectangle {
    length: u16,
    breadth: u16,
}

impl Rectangle {
    fn area(&self) -> u16 {
        self.breadth * self.length
    }

    fn update_len(&mut self, new_len: u16) {
        self.length = new_len;
    }
}

fn main() {
    let mut rec = Rectangle {
        length: 20,
        breadth: 20,
    };

    println!("Area: {}", rec.area());
    println!("Area2: {}", rec.area());

    rec.update_len(30);

    println!("Updated area: {}", rec.area());
}
