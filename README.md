# 🃏 Deck of Cards – Rust Project

A simple Rust CLI program that simulates a basic deck of cards, allowing you to create, shuffle, and deal cards using `Vec<String>` and the `rand` crate.

## 🚀 Features

- Generate a mini deck with suits and values
- Shuffle the deck (coming soon — error handling planned)
- Deal any number of cards from the deck

## 📦 Dependencies

- [`rand`](https://docs.rs/rand/latest/rand/) for RNG and shuffling:
```toml
[dependencies]
rand = "0.8"
```

## 📁 Project Structure

```
deck/
├── Cargo.toml
└── src/
    └── main.rs
```

## 🛠️ How to Run

```bash
cd deck           # Navigate to project directory
cargo run -q      # Run the project quietly
```

You’ll see output like:

```
Here's your hand: [
    "Three of Spades",
    "Two of Hearts",
    "Ace of Clubs"
]
Here's your deck: Deck {
    cards: [
        "Ace of Hearts",
        "Two of Spades",
        ...
    ]
}
```

## 📚 Code Overview

```rust
let suits = ["Hearts", "Spades", "Diamonds", "Clubs"];
let values = ["Ace", "Two", "Three"];
```

This gives us a total of `4 suits x 3 values = 12 cards`.

### Deck Methods:

- `Deck::new()` – initializes the full deck.
- `deck.shuffle()` – randomly shuffles the cards.
- `deck.deal(n)` – deals `n` cards from the top.

> 🔧 Note: Shuffle is present but commented out — and still needs error handling.

## 🧠 Future Improvements

- [ ] Add error handling to shuffle and deal methods
- [ ] Expand card values to include all 13
- [ ] Create a CLI prompt for user interaction
- [ ] Write unit tests

## 👨‍💻 Author

**Anurag Sharma**  
GitHub: [@silverballz](https://github.com/silverballz)
