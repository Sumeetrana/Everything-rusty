struct Container<T> {
    value: T,
}

impl<T: Clone> Container<T> {
    fn new(new_value: T) -> Self {
        Self { value: new_value }
    }

    fn set(&mut self, new_value: T) {
        self.value = new_value;
    }

    fn get(&self) -> T {
        self.value.clone()
    }
}

fn main() {
    let mut container = Container::new(5);
    println!("Value is {}", container.get());

    container.set(10);
    println!("Value is {}", container.get());

    let mut container = Container::new(true);
    println!("Value is {}", container.get());

    container.set(false);
    println!("Value is {}", container.get());
}
