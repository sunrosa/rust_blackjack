use std::{thread, time::Duration};

fn main() {
    let mut deck = deckofcards::Deck::new();
    let mut player_hand = deckofcards::Hand::new();
    let mut dealer_hand = deckofcards::Hand::new();
    let cfg = Config {
        typing_delay: std::time::Duration::from_millis(40),
        typing_line_delay: Duration::from_millis(100),
    };

    deckofcards::Cards::shuffle(&mut deck);
    deck.deal_to_hand(&mut player_hand, 1);
    deck.deal_to_hand(&mut dealer_hand, 1);
    deck.deal_to_hand(&mut player_hand, 1);
    deck.deal_to_hand(&mut dealer_hand, 1);
    type_hand(&player_hand, &cfg);
}

struct Config {
    typing_delay: std::time::Duration,
    typing_line_delay: std::time::Duration,
}

fn typeln(output: &String, delay: std::time::Duration) {
    for c in output.chars() {
        print!("{}", c);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        std::thread::sleep(delay);
    }
    println!();
}

fn type_hand(hand: &deckofcards::Hand, config: &Config) {
    for card in &hand.cards {
        typeln(&card.name(), config.typing_delay);
        thread::sleep(config.typing_line_delay);
    }
    typeln(&format!("{}", hand_value(&hand)), config.typing_delay);
}

fn hand_value(hand: &deckofcards::Hand) -> u8 {
    let mut value: u8 = 0;
    for card in &hand.cards {
        match card.rank {
            deckofcards::Rank::Two => value += 2,
            deckofcards::Rank::Three => value += 3,
            deckofcards::Rank::Four => value += 4,
            deckofcards::Rank::Five => value += 5,
            deckofcards::Rank::Six => value += 6,
            deckofcards::Rank::Seven => value += 7,
            deckofcards::Rank::Eight => value += 8,
            deckofcards::Rank::Nine => value += 9,
            deckofcards::Rank::Ten => value += 10,
            deckofcards::Rank::Jack => value += 10,
            deckofcards::Rank::Queen => value += 10,
            deckofcards::Rank::King => value += 10,
            deckofcards::Rank::Ace => {} // Calculate later in function
        }
    }
    let ace_count: u8 = hand
        .cards
        .iter()
        .filter(|&n| n.rank == deckofcards::Rank::Ace)
        .count()
        .try_into()
        .unwrap();
    for _ in 0..ace_count {
        if value >= 21 {
            value += 1
        } else {
            value += 11
        }
    }
    value
}
