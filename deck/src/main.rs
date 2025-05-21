use rand::{rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>
}

impl Deck{
    fn new() -> Self {
        let suits = ["hearts", "diamonds", "clubs", "spades"];
        let values = ["Ace", "Two", "Three"];
        let mut cards: Vec<String>   = vec![];
    
        for suit in suits {
            for value in values {
                cards.push(format!("{} of {}", value, suit));
            }
        }
    
        //let deck: Deck = Deck {cards};
        Deck{cards}
    }

    fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num: usize) -> Vec<String> {
        return self.cards.split_off(self.cards.len() - num);
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();

    let deal = deck.deal(3);
    
    println!("Deck: {:#?}", deck);
    println!("Dealt: {:#?}", deal);
}
