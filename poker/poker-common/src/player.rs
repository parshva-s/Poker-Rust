struct Player {
    name: String,
    hand: Vec<Card>,
    id: u32,
    total_chips: u32,
    current_bet: u32,
    is_active: bool,
    games_played: Vec<Game>,
    player_stats: Stats,
}

struct Stats {
    games_played: u32,
    games_won: u32,
    games_lost: u32,
    games_folded: u32,
    total_chips_won: u32,
}

impl Player {
    fn new(name: String, id: u32, total_chips: u32) -> Player {
        Player {
            name,
            hand: Vec::new(),
            id,
            total_chips,
            current_bet: 0,
            is_active: false,
        }
    }

    // GETTERS
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_hand(&self) -> &Vec<Card> {
        &self.hand
    }

    fn get_total_chips(&self) -> u32 {
        self.total_chips
    }

    fn is_active(&self) -> bool {
        self.is_active
    }

    fn get_stats(&self) -> &Stats {
        &self.player_stats
    }

    // SETTERS
    fn add_game(&mut self, game: Game) {
        self.games_played.push(game);
    }

    fn add_chips(&mut self, chips: u32) {
        self.total_chips += chips;
    }

    fn remove_chips(&mut self, chips: u32) {
        self.total_chips -= chips;
    }

    fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    fn game_won(&mut self, chips: u32) {
        self.player_stats.games_won += 1;
        self.player_stats.total_chips_won += chips;
        self.add_chips(chips);
        self.player_stats.games_played += 1;
    }

    fn game_lost(&mut self) {
        self.player_stats.games_lost += 1;
        self.player_stats.games_played += 1;
    }

    fn game_folded(&mut self) {
        self.player_stats.games_folded += 1;
        self.player_stats.games_played += 1;
    }

    fn reset_stats(&mut self) {
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
    use crate::stats::PlayerStats;

    #[test]
    fn test_player_new() {
        let mut player = Player::new("John", 1, 1000);
        assert_eq!(player.get_name(), "John");
        assert_eq!(player.get_total_chips(), 1000);
        assert_eq!(player.is_active(), false);
    }
}


