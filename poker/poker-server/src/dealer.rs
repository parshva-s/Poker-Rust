use std::cmp::Ordering;
use std::collections::HashMap;

use poker_common::card::{Card, Suit, Value};
use poker_common::player::Player;
use poker_common::game::{GameSession, Game};

pub struct FiveDrawDealer {
    deck: Vec<Card>,
    dealer_hand: Vec<Card>,
    players: Vec<Player>,
    discard: Vec<Card>,
    pot: u32,
    current_bet: u32,
    current_player: u32,
    round: u32,
}

impl FiveDrawDealer
{
    pub fn new() -> Self {
        FiveDrawDealer {
            deck: Vec::new(),
            players: Vec::new(),
            discard: Vec::new(),
            dealer_hand: Vec::new(),
            pot: 0,
            current_bet: 0,
            current_player: 0,
            round: 0,
        }
    }
}