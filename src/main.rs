mod data;

use std::{thread, time::Duration};

fn main() {
    // Define game state
    let cfg = data::Configuration {
        typing_delay: std::time::Duration::from_millis(15),
        typing_line_delay: Duration::from_millis(75),
        starting_wallet: 100,
        minimum_bet: 5,
    };
    let mut stats = data::Statistics::new(&cfg);

    // Hand loop
    loop {
        if stats.get_wallet() < cfg.minimum_bet {
            typeln(&String::from("Game over!"), &cfg);
            quit(&cfg, &stats);
        }

        // Define game state
        let mut deck = deckofcards::Deck::new();
        let mut player_hand = deckofcards::Hand::new();
        let mut dealer_hand = deckofcards::Hand::new();
        let mut result = GameResult::Unfinished;
        let bet: u32;

        // Initialize game
        deckofcards::Cards::shuffle(&mut deck);
        deck.deal_to_hand(&mut player_hand, 1);
        deck.deal_to_hand(&mut dealer_hand, 1);
        deck.deal_to_hand(&mut player_hand, 1);
        deck.deal_to_hand(&mut dealer_hand, 1);

        typeln(&format!("Wallet: {}", stats.get_wallet()), &cfg);

        // Input bet
        loop {
            print!("Bet (or quit)> ");
            std::io::Write::flush(&mut std::io::stdout()).expect("Could not flush stdio.");

            let mut input = String::new();
            std::io::stdin().read_line(&mut input);
            input = String::from(input.trim());
            match input.parse::<u32>() {
                Ok(i) => {
                    if i < cfg.minimum_bet {
                        println!("You must bet above the minimum bet of {}.", cfg.minimum_bet);
                    } else if i > stats.get_wallet() {
                        println!("You only have {} left in your wallet.", stats.get_wallet())
                    } else {
                        bet = i;
                        stats.decrease_wallet(bet);
                        stats.bet(bet);
                        break;
                    }
                }
                Err(e) => {
                    if input == String::from("quit") {
                        quit(&cfg, &stats);
                    }
                    println!("Please enter a valid integer.")
                }
            }
        }

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
            std::io::Write::flush(&mut std::io::stdout()).expect("Could not flush stdio.");

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
                    } else if hand_value(&player_hand) == 21 && player_hand.cards.len() == 2 {
                        result = GameResult::Blackjack;
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
                    } else if player_hand_value == dealer_hand_value
                        && !(player_hand_value == 21 && player_hand.cards.len() == 2)
                    {
                        result = GameResult::Push;
                    } else if player_hand_value < dealer_hand_value {
                        result = GameResult::Loss;
                    }

                    break;
                }
                "help" => {
                    println!("hit: Receive an additional card for your hand.\nstand: Keep your current hand and advance the game.\nhelp: Print help.\nquit: Quit the program.")
                }
                "quit" => quit(&cfg, &stats),
                _ => println!("Invalid command! Type \"help\" for more information."),
            }
        }

        match result {
            GameResult::Win => {
                typeln(&String::from("Win!"), &cfg);
                stats.increase_wallet(bet * 2);
                stats.pure_win();
            }
            GameResult::Loss => {
                typeln(&String::from("Loss!"), &cfg);
                stats.pure_loss();
            }
            GameResult::Blackjack => {
                typeln(&String::from("Blackjack!!!"), &cfg);
                stats.increase_wallet(bet * 2);
                stats.blackjack();
            }
            GameResult::Bust => {
                typeln(&String::from("Bust!"), &cfg);
                stats.bust();
            }
            GameResult::Push => {
                typeln(&String::from("Push!"), &cfg);
                stats.increase_wallet(bet);
                stats.push();
            }
            GameResult::DealerBust => {
                typeln(&String::from("Dealer bust!"), &cfg);
                stats.increase_wallet(bet * 2);
                stats.dealer_bust();
            }
            GameResult::Unfinished => {
                panic!(
                    "Game result should not be unfinished upon exit from game loop, and yet it is."
                )
            }
        }
        println!("================");
    }
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

fn typeln(output: &String, config: &data::Configuration) {
    for c in output.chars() {
        print!("{}", c);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        std::thread::sleep(config.typing_delay);
    }
    println!();
    std::thread::sleep(config.typing_line_delay);
}

fn type_hand(hand: &deckofcards::Hand, config: &data::Configuration) {
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

fn quit(config: &data::Configuration, stats: &data::Statistics) {
    // Print statistics
    typeln(
        &String::from(format!(
            "Final wallet: {wallet}\nHands played: {handsplayed}\nTotal won: {totalwon} / Total bet: {totalbet}\nAverage bet: {averagebet}\nWins: {wins} / Draws: {draws} / Losses: {losses}\nPure wins: {pwins} / Pure losses: {plosses}\nBlackjacks: {blackjacks}\nBusts: {busts}\nDealer busts: {dbusts}",
            wallet = stats.get_wallet(),
            handsplayed = stats.get_hands_played(),
            totalwon = stats.get_total_won(),
            totalbet = stats.get_total_bet(),
            averagebet = format!("{:.2}", stats.get_average_bet()),
            wins = stats.get_total_wins(),
            draws = stats.get_total_draws(),
            losses = stats.get_total_losses(),
            pwins = stats.get_total_pure_wins(),
            plosses = stats.get_total_pure_losses(),
            blackjacks = stats.get_total_blackjacks(),
            busts = stats.get_total_busts(),
            dbusts = stats.get_total_dealer_busts()
        )),
        &config,
    );

    std::process::exit(0);
}
