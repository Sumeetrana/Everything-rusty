struct Counter {
    counter: u32,
}

impl Counter {
    fn new() -> Self {
        Counter { counter: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.counter = self.counter + 1;

        if self.counter < 5 {
            Some(self.counter)
        } else {
            None
        }
    }
}

fn main() {
    let mut counter = Counter::new();

    while let Some(value) = counter.next() {
        println!("{}", value);
    }
}
