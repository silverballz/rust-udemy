#[derive(Debug)]
struct Deck {
    #[allow(dead_code)]
    cards: Vec<String>,
}

fn main() {
    let deck = Deck { cards: vec![] };

    println!("Here's your deck: {:?}", deck);
}