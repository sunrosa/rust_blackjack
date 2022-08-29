use std::{thread, time::Duration};

fn main() {
    // Define game state
    let cfg = Config {
        typing_delay: std::time::Duration::from_millis(15),
        typing_line_delay: Duration::from_millis(75),
    };

    // Hand loop
    loop {
        // Define game state
        let mut deck = deckofcards::Deck::new();
        let mut player_hand = deckofcards::Hand::new();
        let mut dealer_hand = deckofcards::Hand::new();
        let mut result = GameResult::Unfinished;

        // Initialize game
        deckofcards::Cards::shuffle(&mut deck);
        deck.deal_to_hand(&mut player_hand, 1);
        deck.deal_to_hand(&mut dealer_hand, 1);
        deck.deal_to_hand(&mut player_hand, 1);
        deck.deal_to_hand(&mut dealer_hand, 1);

        // Print initial game state to user
        typeln(&String::from("Your hand:"), &cfg);
        type_hand(&player_hand, &cfg); // Print all cards in player hand
        typeln(&String::from("Dealer hand:"), &cfg);
        type_hand(
            &deckofcards::Hand::from_cards(&dealer_hand.cards[0..1]),
            &cfg,
        ); // Print first card only in dealer's hand

        // Command loop
        loop {
            // Print prompt
            print!("> ");
            std::io::Write::flush(&mut std::io::stdout());

            // Read and format user input
            let mut input = String::new();
            std::io::stdin().read_line(&mut input);
            input = String::from(input.trim());

            match input.as_str() {
                "hit" => {
                    deck.deal_to_hand(&mut player_hand, 1);
                    type_hand(&player_hand, &cfg);

                    if hand_value(&player_hand) > 21 {
                        result = GameResult::Bust;
                        break;
                    }
                }
                "stand" => {
                    typeln(&String::from("Dealer hand:"), &cfg);

                    // Dealer hit below 16
                    while hand_value(&dealer_hand) <= 16 {
                        deck.deal_to_hand(&mut dealer_hand, 1);
                    }

                    type_hand(&dealer_hand, &cfg);

                    let player_hand_value = hand_value(&player_hand);
                    let dealer_hand_value = hand_value(&dealer_hand);
                    if player_hand_value > 21 {
                        result = GameResult::Bust;
                    } else if player_hand_value == 21 && player_hand.cards.len() == 2 {
                        result = GameResult::Blackjack;
                    } else if dealer_hand_value > 21 && player_hand_value <= 21 {
                        result = GameResult::DealerBust;
                    } else if player_hand_value > dealer_hand_value && player_hand_value <= 21 {
                        result = GameResult::Win;
                    } else if player_hand_value == dealer_hand_value {
                        result = GameResult::Push;
                    } else if player_hand_value < dealer_hand_value {
                        result = GameResult::Loss;
                    }

                    break;
                }
                "help" => {
                    println!("hit: Receive an additional card for your hand.\nstand: Keep your current hand and advance the game.\nhelp: Print help.\nquit: Quit the program.")
                }
                "quit" => quit(),
                _ => println!("Invalid command! Type \"help\" for more information."),
            }
        }

        match result {
            GameResult::Win => typeln(&String::from("Win!"), &cfg),
            GameResult::Loss => typeln(&String::from("Loss!"), &cfg),
            GameResult::Blackjack => typeln(&String::from("Blackjack!!!"), &cfg),
            GameResult::Bust => typeln(&String::from("Bust!"), &cfg),
            GameResult::Push => typeln(&String::from("Push!"), &cfg),
            GameResult::DealerBust => typeln(&String::from("Dealer bust!"), &cfg),
            GameResult::Unfinished => {
                panic!(
                    "Game result should not be unfinished upon exit from game loop, and yet it is."
                )
            }
        }
        println!("================")
    }
}

struct Config {
    typing_delay: std::time::Duration,
    typing_line_delay: std::time::Duration,
}

enum GameResult {
    Win,
    Loss,
    Blackjack,
    Bust,
    Push,
    DealerBust,
    Unfinished,
}

fn typeln(output: &String, config: &Config) {
    for c in output.chars() {
        print!("{}", c);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        std::thread::sleep(config.typing_delay);
    }
    println!();
    std::thread::sleep(config.typing_line_delay);
}

fn type_hand(hand: &deckofcards::Hand, config: &Config) {
    typeln(&format!("({})", hand_value(&hand)), &config);
    for card in &hand.cards {
        typeln(&card.name(), &config);
        thread::sleep(config.typing_line_delay);
    }
    typeln(&String::new(), &config);
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
        if value + 11 > 21 {
            value += 1
        } else {
            value += 11
        }
    }
    value
}

fn quit() {
    std::process::exit(0);
}
