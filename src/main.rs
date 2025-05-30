#[derive(Debug)]
struct Deck {
    #[allow(dead_code)]
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamonds", "Clubs"];
        let values = ["Ace", "Two", "Three"];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        Deck { cards } // returned implicitly
    }
}

fn main() {
    let deck = Deck::new();

    println!("Here's your deck: {:#?}", deck);
}