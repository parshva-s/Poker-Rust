use crate::player::Player;

pub struct Game {
    id: u32,
    players: Vec<Player>,
    winning_players: Vec<Player>,
    total_chips: u32,
}

pub struct GameSession {
    game_session_id: u32,
    games: Vec<Game>,
    current_game: Option<Game>,
}

impl Game {
    pub fn new(id: u32, players: Vec<Player>) -> Game {
        Game {
            id,
            players,
            winning_players: Vec::new(),
            total_chips: 0,
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn remove_player(&mut self, player: Player) {
        self.players.retain(|p| p.get_name() != player.get_name());
    }

    pub fn get_players(&self) -> &Vec<Player> {
        &self.players
    }

    pub fn get_winning_players(&self) -> &Vec<Player> {
        &self.winning_players
    }

    pub fn get_total_chips(&self) -> u32 {
        self.total_chips
    }

    pub fn set_total_chips(&mut self, chips: u32) {
        self.total_chips = chips;
    }

    pub fn add_winning_player(&mut self, player: Player) {
        self.winning_players.push(player);
    }
}

impl GameSession {
    pub fn new(session_id: u32) -> GameSession {
        GameSession {
            game_session_id: session_id,
            games: Vec::new(),
            current_game: None,
        }
    }

    pub fn start_game(&mut self, players: Vec<Player>) {
        let game = Game {
            id: self.games.len() as u32,
            players,
            winning_players: Vec::new(),
            total_chips: 0,
        };
        self.current_game = Some(game);
    }

    pub fn end_game(&mut self) {
        if let Some(game) = self.current_game.take() {
            self.games.push(game);
        }
    }

    pub fn get_current_game(&self) -> Option<&Game> {
        self.current_game.as_ref()
    }

    pub fn get_games(&self) -> &Vec<Game> {
        &self.games
    }
}
