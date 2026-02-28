use rand::{rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        // List of 'suits' - 'Hearts', 'Diamonds', 'Clubs', 'Spades'
        let suits = ["Hearts", "Spades", "Diamonds", "Clubs"];

        // List of 'values' - '2', '3', '4', '5', '6', '7', '8', '9', '10', 'Jack', 'Queen', 'King', 'Ace'
        let values = [
            "Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King",
        ];

        let mut cards = vec![];

        // Double nested for loop to create a deck of cards
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck { cards }
    }

    fn shuffle(&mut self) {
        // Shuffle the deck of cards
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }
}

fn main() {
    let mut deck = Deck::new();

    deck.shuffle();

    println!("Heres your deck: {:#?}", deck.cards);
}
