mod data;

use std::{thread, time::Duration};

use deckofcards::Card;

fn main() {
    // Define game state
    let cfg = data::Configuration {
        typing_delay: std::time::Duration::from_millis(15),
        typing_line_delay: Duration::from_millis(75),
        starting_wallet: 100,
        minimum_bet: 5,
        currency_prefix: String::from("$"),
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
        let mut player_hands = vec![deckofcards::Hand::new()];
        let mut dealer_hand = deckofcards::Hand::new();
        let mut result = data::GameResult::Unfinished;
        let mut result_split: data::GameResult;
        let mut bet: u32;

        // Initialize game
        deckofcards::Cards::shuffle(&mut deck);
        deck.deal_to_hand(&mut player_hands[0], 1);
        deck.deal_to_hand(&mut dealer_hand, 1);
        deck.deal_to_hand(&mut player_hands[0], 1);
        deck.deal_to_hand(&mut dealer_hand, 1);

        typeln(
            &format!("Wallet: {}{}", &cfg.currency_prefix, stats.get_wallet()),
            &cfg,
        );

        // Input bet
        loop {
            print!("Bet (or quit)> ");
            std::io::Write::flush(&mut std::io::stdout()).expect("Could not flush stdio.");

            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Could not read line from stdin.");
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
                Err(_) => {
                    if input == String::from("quit") {
                        quit(&cfg, &stats);
                    }
                    println!("Please enter a valid integer.")
                }
            }
        }

        // Print initial game state to user
        typeln(&String::from("Your hand:"), &cfg);
        type_hand(&player_hands[0], &cfg); // Print all cards in player hand
        typeln(&String::from("Dealer hand:"), &cfg);
        type_hand(
            &deckofcards::Hand::from_cards(&dealer_hand.cards[0..1]),
            &cfg,
        ); // Print first card only in dealer's hand

        if hand_value(&player_hands[0]) == 21 && player_hands[0].cards.len() as u8 == 2 {
            result = data::GameResult::Blackjack;
        }
        // Command loop
        if result == data::GameResult::Unfinished {
            loop {
                // Print prompt
                print!("> ");
                std::io::Write::flush(&mut std::io::stdout()).expect("Could not flush stdio.");

                // Read and format user input
                let mut input = String::new();
                std::io::stdin()
                    .read_line(&mut input)
                    .expect("Error reading line from stdin.");
                input = String::from(input.trim());

                match input.as_str() {
                    "hit" => {
                        deck.deal_to_hand(&mut player_hands[0], 1);
                        type_hand(&player_hands[0], &cfg);

                        if hand_value(&player_hands[0]) > 21 {
                            result = data::GameResult::Bust;
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

                        let player_hand_value = hand_value(&player_hands[0]);
                        let dealer_hand_value = hand_value(&dealer_hand);
                        if player_hand_value > 21 {
                            result = data::GameResult::Bust;
                        } else if player_hand_value == 21 && player_hands[0].cards.len() == 2 {
                            result = data::GameResult::Blackjack;
                        } else if dealer_hand_value > 21 && player_hand_value <= 21 {
                            result = data::GameResult::DealerBust;
                        } else if player_hand_value > dealer_hand_value && player_hand_value <= 21 {
                            result = data::GameResult::Win;
                        } else if player_hand_value == dealer_hand_value
                            && !(player_hand_value == 21 && player_hands[0].cards.len() == 2)
                        {
                            result = data::GameResult::Push;
                        } else if player_hand_value < dealer_hand_value {
                            result = data::GameResult::Loss;
                        }

                        break;
                    }
                    "double" => {
                        // Exit if the player has already hit
                        if player_hands[0].cards.len() != 2 {
                            println!("Doubling down is only allowed before hitting.");
                            continue;
                        }

                        // Double the bet
                        stats.decrease_wallet(bet);
                        stats.bet(bet);
                        bet *= 2;

                        // Deal one additional card
                        deck.deal_to_hand(&mut player_hands[0], 1);
                        type_hand(&player_hands[0], &cfg);

                        // Bust automatically if over 21
                        if hand_value(&player_hands[0]) > 21 {
                            result = data::GameResult::Bust;
                            break;
                        }
                    }
                    /*"split" => {
                        if player_hands.cards.len() != 2
                            || card_value(&player_hands.cards.get(0).unwrap())
                                != card_value(&player_hands.cards.get(1).unwrap())
                        {
                            println!("You may only split when you have two cards of equal rank.")
                        }

                        // Split the hands in two
                        player_hands.remove(0);
                        player_hand_split = player_hands.clone();

                        // Deal one card to each hand
                        deck.deal_to_hand(&mut player_hands, 1);
                        deck.deal_to_hand(&mut player_hand_split, 1);
                    }*/
                    "help" => {
                        println!("hit: Receive an additional card for your hand.\nstand: Keep your current hand and advance the game.\ndouble: Double your bet and gain an additional card (can only be used before hitting for the first time).\nsplit: Split first two cards of same value into two separate hands, betting additionally for your second hand.\nhelp: Print help.\nquit: Quit the program.")
                    }
                    "quit" => quit(&cfg, &stats),
                    _ => println!("Invalid command! Type \"help\" for more information."),
                }
            }
        }

        match result {
            data::GameResult::Win => {
                typeln(&format!("Win! +({}{})", cfg.currency_prefix, bet), &cfg);
                stats.increase_wallet(bet * 2);
                stats.pure_win();
            }
            data::GameResult::Loss => {
                typeln(&format!("Loss! -({}{})", cfg.currency_prefix, bet), &cfg);
                stats.pure_loss();
            }
            data::GameResult::Blackjack => {
                typeln(
                    &format!("Blackjack!!! +({}{})", cfg.currency_prefix, bet),
                    &cfg,
                );
                stats.increase_wallet(bet * 2);
                stats.blackjack();
            }
            data::GameResult::Bust => {
                typeln(&format!("Bust! -({}{})", cfg.currency_prefix, bet), &cfg);
                stats.bust();
            }
            data::GameResult::Push => {
                typeln(&String::from("Push!"), &cfg);
                stats.increase_wallet(bet);
                stats.push();
            }
            data::GameResult::DealerBust => {
                typeln(
                    &format!("Dealer bust! +({}{})", cfg.currency_prefix, bet),
                    &cfg,
                );
                stats.increase_wallet(bet * 2);
                stats.dealer_bust();
            }
            data::GameResult::Unfinished => {
                panic!(
                    "Game result should not be unfinished upon exit from game loop, and yet it is."
                )
            }
        }
        println!("================");
    }
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

fn card_value(card: &Card) -> u8 {
    match card.rank {
        deckofcards::Rank::Two => 2,
        deckofcards::Rank::Three => 3,
        deckofcards::Rank::Four => 4,
        deckofcards::Rank::Five => 5,
        deckofcards::Rank::Six => 6,
        deckofcards::Rank::Seven => 7,
        deckofcards::Rank::Eight => 8,
        deckofcards::Rank::Nine => 9,
        deckofcards::Rank::Ten => 10,
        deckofcards::Rank::Jack => 10,
        deckofcards::Rank::Queen => 10,
        deckofcards::Rank::King => 10,
        deckofcards::Rank::Ace => 11,
    }
}
fn quit(config: &data::Configuration, stats: &data::Statistics) {
    // Print statistics
    typeln(
        &String::from(format!(
            "Final wallet: {cur_prefix}{wallet}\nHands played: {handsplayed}\nTotal won: {cur_prefix}{totalwon} / Total bet: {cur_prefix}{totalbet}\nAverage bet: {cur_prefix}{averagebet}\nWins: {wins} / Draws: {draws} / Losses: {losses}\nPure wins: {pwins} / Pure losses: {plosses}\nBlackjacks: {blackjacks}\nBusts: {busts}\nDealer busts: {dbusts}",
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
            dbusts = stats.get_total_dealer_busts(),
            cur_prefix = config.currency_prefix
        )),
        &config,
    );

    std::process::exit(0);
}
