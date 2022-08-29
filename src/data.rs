pub struct Statistics {
    wallet: u32,
    hands_played: u32,
    total_bet: u32,
    total_won: u32,
    total_wins: u32,
    total_losses: u32,
    total_pure_wins: u32,
    total_pure_losses: u32,
    total_blackjacks: u32,
    total_busts: u32,
    total_draws: u32,
    total_dealer_busts: u32,
    average_bet: f32,
}

impl Statistics {
    pub fn new(config: &Configuration) -> Statistics {
        Statistics {
            wallet: config.starting_wallet,
            hands_played: 0,
            total_bet: 0,
            total_won: 0,
            total_wins: 0,
            total_losses: 0,
            total_pure_wins: 0,
            total_pure_losses: 0,
            total_blackjacks: 0,
            total_busts: 0,
            total_draws: 0,
            total_dealer_busts: 0,
            average_bet: 0.0,
        }
    }

    pub fn increase_wallet(&mut self, amount: u32) {
        self.wallet += amount;
        self.total_won += amount;
    }
    pub fn decrease_wallet(&mut self, amount: u32) {
        self.wallet -= amount;
        self.total_bet += amount;
    }
    pub fn pure_win(&mut self) {
        self.total_wins += 1;
        self.total_pure_wins += 1;
        self.hands_played += 1;
    }
    pub fn pure_loss(&mut self) {
        self.total_losses += 1;
        self.total_pure_losses += 1;
        self.hands_played += 1;
    }
    pub fn blackjack(&mut self) {
        self.total_wins += 1;
        self.total_blackjacks += 1;
        self.hands_played += 1;
    }
    pub fn bust(&mut self) {
        self.total_losses += 1;
        self.total_busts += 1;
        self.hands_played += 1;
    }
    pub fn push(&mut self) {
        self.total_draws += 1;
        self.hands_played += 1;
    }
    pub fn dealer_bust(&mut self) {
        self.total_wins += 1;
        self.total_dealer_busts += 1;
        self.hands_played += 1;
    }
    pub fn bet(&mut self, amount: u32) {
        self.average_bet = ((self.hands_played as f32 * self.average_bet) + amount as f32)
            / (self.hands_played as f32 + 1.0)
    }
    pub fn get_wallet(&self) -> u32 {
        self.wallet
    }
    pub fn get_hands_played(&self) -> u32 {
        self.hands_played
    }
    pub fn get_total_won(&self) -> u32 {
        self.total_won
    }
    pub fn get_total_bet(&self) -> u32 {
        self.total_bet
    }
    pub fn get_average_bet(&self) -> f32 {
        self.average_bet
    }
    pub fn get_total_wins(&self) -> u32 {
        self.total_wins
    }
    pub fn get_total_losses(&self) -> u32 {
        self.total_losses
    }
    pub fn get_total_pure_wins(&self) -> u32 {
        self.total_pure_wins
    }
    pub fn get_total_pure_losses(&self) -> u32 {
        self.total_pure_losses
    }
    pub fn get_total_blackjacks(&self) -> u32 {
        self.total_blackjacks
    }
    pub fn get_total_busts(&self) -> u32 {
        self.total_busts
    }
    pub fn get_total_dealer_busts(&self) -> u32 {
        self.total_dealer_busts
    }
}

pub struct Configuration {
    pub typing_delay: std::time::Duration,
    pub typing_line_delay: std::time::Duration,
    pub starting_wallet: u32,
    pub minimum_bet: u32,
}
