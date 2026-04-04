#[derive(Debug)]
struct Pair<T, U> {
    first: T,
    second: U,
}

trait PairMethods<T, U> {
    fn getFirstElement(&self) -> &T;
    fn getSecondElement(&self) -> &U;
    fn swap(&self) -> Pair<&U, &T>;
}

impl<T, U> PairMethods<T, U> for Pair<T, U> {
    fn getFirstElement(&self) -> &T {
        &self.first
    }

    fn getSecondElement(&self) -> &U {
        &self.second
    }

    fn swap(&self) -> Pair<&U, &T> {
        Pair {
            first: &self.second,
            second: &self.first,
        }
    }
}

fn main() {
    let pair = Pair {
        first: 10,
        second: 20,
    };

    println!("{:#?}", pair);

    println!("First element: {}", pair.getFirstElement());
    println!("Second element: {}", pair.getSecondElement());

    println!("Swapped  {:#?}", pair.swap());
}
