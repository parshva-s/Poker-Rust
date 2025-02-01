struct Game {
    id: u32,
    players: Vec<Player>,
    winning_players: Vec<Player>,
    total_chips: u32,
}

struct GameSession {
    game_session_id: u32,
    games: Vec<Game>,
    current_game: Option<Game>,
}

impl Game {
    fn new(id: u32, players: Vec<Player>) -> Game {
        Game {
            id,
            players,
            winning_players: Vec::new(),
            total_chips: 0,
        }
    }

    fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    fn remove_player(&mut self, player: Player) {
        self.players.retain(|p| p != &player);
    }

    fn get_players(&self) -> &Vec<Player> {
        &self.players
    }

    fn get_winning_players(&self) -> &Vec<Player> {
        &self.winning_players
    }

    fn get_total_chips(&self) -> u32 {
        self.total_chips
    }

    fn set_total_chips(&mut self, chips: u32) {
        self.total_chips = chips;
    }

    fn add_winning_player(&mut self, player: Player) {
        self.winning_players.push(player);
    }
}

impl GameSession {
    fn new(session_id: u32) -> GameSession {
        GameSession {
            game_session_id: session_id,
            games: Vec::new(),
            current_game: None,
        }
    }

    fn start_game(&mut self, players: Vec<Player>) {
        let game = Game {
            id: self.games.len() as u32,
            players,
            winning_players: Vec::new(),
            total_chips: 0,
        };
        self.current_game = Some(game);
    }

    fn end_game(&mut self) {
        if let Some(game) = self.current_game.take() {
            self.games.push(game);
        }
    }

    fn get_current_game(&self) -> Option<&Game> {
        self.current_game.as_ref()
    }

    fn get_games(&self) -> &Vec<Game> {
        &self.games
    }
}
