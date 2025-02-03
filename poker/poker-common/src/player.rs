use crate::game::Game;
use crate::card::{Card, Suit, Value};

pub struct Player {
    name: String,
    hand: Vec<Card>,
    id: u32,
    total_chips: u32,
    current_bet: u32,
    is_active: bool,
    games_played: Vec<Game>,
    player_stats: Stats,
}

pub struct Stats {
    games_played: u32,
    games_won: u32,
    games_lost: u32,
    games_folded: u32,
    total_chips_won: u32,
}

impl Player {
    pub fn new(name: String, id: u32, total_chips: u32) -> Player {
        Player {
            name,
            hand: Vec::new(),
            id,
            total_chips,
            current_bet: 0,
            games_played: Vec::new(),
            player_stats: Stats {
                games_played: 0,
                games_won: 0,
                games_lost: 0,
                games_folded: 0,
                total_chips_won: 0,
            },
            is_active: false,
        }
    }

    // GETTERS
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_hand(&self) -> &Vec<Card> {
        &self.hand
    }

    pub fn get_total_chips(&self) -> u32 {
        self.total_chips
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn get_stats(&self) -> &Stats {
        &self.player_stats
    }

    // SETTERS
    pub fn add_game(&mut self, game: Game) {
        self.games_played.push(game);
    }

    pub fn add_chips(&mut self, chips: u32) {
        self.total_chips += chips;
    }

    pub fn remove_chips(&mut self, chips: u32) {
        self.total_chips -= chips;
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn add_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    pub fn clear_hand(&mut self) {
        self.hand.clear();
    }

    pub fn remove_card(&mut self, suit: Suit, value: Value) -> bool{
        let index = self.hand.iter().position(|card| card.suit == suit && card.value == value);
        if let Some(index) = index {
            self.hand.remove(index);
            return true;
        }
        return false;
    }

    pub fn game_won(&mut self, chips: u32) {
        self.player_stats.games_won += 1;
        self.player_stats.total_chips_won += chips;
        self.add_chips(chips);
        self.player_stats.games_played += 1;
    }

    pub fn game_lost(&mut self) {
        self.player_stats.games_lost += 1;
        self.player_stats.games_played += 1;
    }

    pub fn game_folded(&mut self) {
        self.player_stats.games_folded += 1;
        self.player_stats.games_played += 1;
    }

    pub fn reset_stats(&mut self) {
        self.player_stats.games_played = 0;
        self.player_stats.games_won = 0;
        self.player_stats.games_lost = 0;
        self.player_stats.games_folded = 0;
        self.player_stats.total_chips_won = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Game;
    use crate::player::Player;
    use crate::player::Stats;

    #[test]
    fn test_player_new() {
        let player = Player::new("John".to_owned(), 1, 1000);
        assert_eq!(player.get_name(), "John");
        assert_eq!(player.get_total_chips(), 1000);
        assert_eq!(player.is_active(), false);
    }
}


