use std::cmp::Ordering;
use std::collections::HashMap;
use rand::prelude::*;

use poker_common::card::{Card, Suit, Value};
use poker_common::player::{Player};
use poker_common::game::{GameSession, Game};

pub struct FiveDrawDealer {
    deck: Vec<Card>,
    cards_in_deck: u32,
    small_blind: u32,
    big_blind: u32,
    dealer_hand: Vec<Card>,
    players: Vec<Player>,
    pot: u32,
    current_bet: u32,
    current_player: u32,
    last_bet_player: u32,
    round: u32,
}

impl FiveDrawDealer
{
    pub fn new() -> Self {
        FiveDrawDealer {
            deck: Vec::new(),
            players: Vec::new(),
            dealer_hand: Vec::new(),
            pot: 0,
            small_blind: 0,
            big_blind: 1,
            current_bet: 0,
            current_player: 0,
            round: 0,
            last_bet_player: 0,
            cards_in_deck: 52,
        }
    }

    pub fn start_game(&mut self) {
        self.round += 1;

        self.create_deck();
        self.deal_initial_hand();
        
        self.small_blind = self.big_blind % self.players.len() as u32;
        self.big_blind = (self.big_blind + 1) % self.players.len() as u32;
        self.last_bet_player = self.big_blind;

        // make all players active
        for player in self.players.iter_mut() {
            player.joined_table();
        }

        self.current_player = (self.big_blind + 1) % self.players.len() as u32;
    }

    pub fn deal_initial_hand(&mut self) {
        if self.players.len() < 2 {
            return; // No players, nothing to deal
        }
    
        // Ensure the deck is created and shuffled before dealing
        self.create_deck();
        let mut rng = rand::thread_rng();
        self.deck.shuffle(&mut rng);
    
        // Collect mutable references to players to avoid multiple mutable borrows of `self`
        let mut player_refs: Vec<*mut Player> = self.players.iter_mut().map(|p| p as *mut Player).collect();

        // Deal five cards to each player using deal_card method
        for _ in 0..5 {
            for &player_ptr in &player_refs {
                let player = unsafe { &mut *player_ptr };
                self.deal_card(player);
            }
        }
    }

    pub fn create_deck(&mut self) {
        self.deck.clear();
        for suit in &[Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades] {
            for value in &[Value::Two, Value::Three, Value::Four, Value::Five, Value::Six, Value::Seven, Value::Eight, Value::Nine, Value::Ten, Value::Jack, Value::Queen, Value::King, Value::Ace] {
                self.deck.push(Card {suit: suit.clone(), value: value.clone()});
            }
        }
        self.cards_in_deck = 52;
    }

    pub fn deal_card(&mut self, player: &mut Player) -> Option<Card> {
        if self.cards_in_deck == 0 {
            return None;
        }

        // pop a random card from the deck and give it to the player
        let random_index = rand::thread_rng().gen_range(0, self.deck.len());
        let card = self.deck.remove(random_index);

        player.add_card(card.clone());
        self.cards_in_deck -= 1;
        Some(card)
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn check_for_winning_hand(&self) -> Vec<&Player> {
        if self.players.is_empty() {
            return vec![]; // No players, no winner
        }

        // Find the highest-ranked hand
        let best_hand = self.players.iter().map(|p| {
            let hand = p.get_hand();
            (p, Self::evaluate_hand(hand))
        }).max_by(|(_, (rank1, values1)), (_, (rank2, values2))| {
            match rank1.cmp(rank2) {
                Ordering::Greater => Ordering::Greater,
                Ordering::Less => Ordering::Less,
                Ordering::Equal => values1.cmp(values2),
            }
        });

        if let Some((_, best_eval)) = best_hand {
            // Filter all players with the best hand rank and values
            self.players.iter()
                .filter(|p| Self::evaluate_hand(p.get_hand()) == best_eval)
                .collect()
        } else {
            vec![] // No winner case (shouldn't happen with valid hands)
        }
    }

    fn evaluate_hand(hand: &[Card]) -> (u8, Vec<u8>) {
        let mut values: Vec<u8> = hand.iter().map(|c| c.value.rank()).collect();
        let mut suits: Vec<Suit> = hand.iter().map(|c| c.suit.clone()).collect();
        let mut value_counts: HashMap<u8, u8> = HashMap::new();
    
        for &v in &values {
            *value_counts.entry(v).or_insert(0) += 1;
        }
    
        values.sort_by(|a, b| b.cmp(a)); // Sort highest to lowest for tiebreaker
    
        let is_flush = suits.iter().all(|s| *s == suits[0]);
        let is_straight = values.windows(2).all(|w| w[1] == w[0] - 1);
        let ace_low_straight = values == vec![14, 5, 4, 3, 2]; // A-2-3-4-5 case
    
        let mut counts: Vec<(u8, u8)> = value_counts.iter().map(|(&v, &c)| (c, v)).collect();
        counts.sort_by(|a, b| b.cmp(a)); // Sort by count then value
    
        let hand_rank = if is_flush && (is_straight || ace_low_straight) {
            if values.contains(&14) { 10 } else { 9 } // Royal Flush or Straight Flush
        } else if counts[0].0 == 4 {
            8 // Four of a Kind
        } else if counts[0].0 == 3 && counts[1].0 == 2 {
            7 // Full House
        } else if is_flush {
            6 // Flush
        } else if is_straight || ace_low_straight {
            5 // Straight
        } else if counts[0].0 == 3 {
            4 // Three of a Kind
        } else if counts.len() > 1 && counts[0].0 == 2 && counts[1].0 == 2 {
            3 // Two Pair
        } else if counts[0].0 == 2 {
            2 // One Pair
        } else {
            1 // High Card
        };
    
        // Create tiebreaker sequence (first by hand type, then high cards)
        let mut tiebreaker_values: Vec<u8> = counts.iter().map(|(_, v)| *v).collect();
        tiebreaker_values.extend(values);
    
        (hand_rank, tiebreaker_values)
    }

    fn compare_hands(hand1: &[Card], hand2: &[Card]) -> Ordering {
        let (rank1, values1) = Self::evaluate_hand(hand1);
        let (rank2, values2) = Self::evaluate_hand(hand2);
    
        match rank1.cmp(&rank2) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                for (v1, v2) in values1.iter().zip(values2.iter()) {
                    match v1.cmp(v2) {
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => continue,
                    }
                }
                Ordering::Equal
            }
        }
    }    
}

#[cfg(test)]
mod tests {
    use super::*;
    use poker_common::card::{Card, Suit, Value};
    use poker_common::player::Player;

    #[test]
    fn test_evaluate_hand() {
        let player1 = Player::new("John".to_owned(), 1, 1000);
        let player2 = Player::new("Jane".to_owned(), 2, 1000);
        let player3 = Player::new("Bob".to_owned(), 3, 1000);

        let mut dealer = FiveDrawDealer::new();
        dealer.add_player(player1);
        dealer.add_player(player2);
        dealer.add_player(player3);

        // give player 1 a pair of 2s and 3 random cards
        dealer.players[0].add_card(Card {suit: Suit::Hearts, value: Value::Two});
        dealer.players[0].add_card(Card {suit: Suit::Diamonds, value: Value::Two});
        dealer.players[0].add_card(Card {suit: Suit::Clubs, value: Value::Five});
        dealer.players[0].add_card(Card {suit: Suit::Spades, value: Value::Nine});
        dealer.players[0].add_card(Card {suit: Suit::Hearts, value: Value::King});

        // give player 2 a pair of 3s and 3 random cards
        dealer.players[1].add_card(Card {suit: Suit::Diamonds, value: Value::Three});
        dealer.players[1].add_card(Card {suit: Suit::Clubs, value: Value::Three});
        dealer.players[1].add_card(Card {suit: Suit::Spades, value: Value::Eight});
        dealer.players[1].add_card(Card {suit: Suit::Hearts, value: Value::Four});
        dealer.players[1].add_card(Card {suit: Suit::Diamonds, value: Value::Seven});

        // give player 3 a pair of 4s and 3 random cards
        dealer.players[2].add_card(Card {suit: Suit::Clubs, value: Value::Four});
        dealer.players[2].add_card(Card {suit: Suit::Spades, value: Value::Four});
        dealer.players[2].add_card(Card {suit: Suit::Hearts, value: Value::Ten});
        dealer.players[2].add_card(Card {suit: Suit::Diamonds, value: Value::Six});
        dealer.players[2].add_card(Card {suit: Suit::Clubs, value: Value::Queen});

        let winner = dealer.check_for_winning_hand();
        assert_eq!(winner.len(), 1);
        assert_eq!(winner[0].get_name(), "Bob");
    }

    #[test]
    fn evaluate_same_hand() {
        let player1 = Player::new("John".to_owned(), 1, 1000);
        let player2 = Player::new("Jane".to_owned(), 2, 1000);
        let player3 = Player::new("Bob".to_owned(), 3, 1000);

        let mut dealer = FiveDrawDealer::new();

        dealer.add_player(player1);
        dealer.add_player(player2);
        dealer.add_player(player3);

        // give player 1 a pair of 2s and 3 random cards
        dealer.players[0].add_card(Card {suit: Suit::Hearts, value: Value::Two});
        dealer.players[0].add_card(Card {suit: Suit::Diamonds, value: Value::Two});
        dealer.players[0].add_card(Card {suit: Suit::Clubs, value: Value::Five});
        dealer.players[0].add_card(Card {suit: Suit::Spades, value: Value::Nine});
        dealer.players[0].add_card(Card {suit: Suit::Hearts, value: Value::King});

        // give player 2 a pair of 2s and 3 random cards
        dealer.players[1].add_card(Card {suit: Suit::Spades, value: Value::Two});
        dealer.players[1].add_card(Card {suit: Suit::Clubs, value: Value::Two});
        dealer.players[1].add_card(Card {suit: Suit::Hearts, value: Value::Five});
        dealer.players[1].add_card(Card {suit: Suit::Diamonds, value: Value::Nine});
        dealer.players[1].add_card(Card {suit: Suit::Spades, value: Value::King});

        // give player 3 a pair of 5 random cards
        dealer.players[2].add_card(Card {suit: Suit::Hearts, value: Value::Three});
        dealer.players[2].add_card(Card {suit: Suit::Diamonds, value: Value::Four});
        dealer.players[2].add_card(Card {suit: Suit::Clubs, value: Value::Five});
        dealer.players[2].add_card(Card {suit: Suit::Spades, value: Value::Six});
        dealer.players[2].add_card(Card {suit: Suit::Hearts, value: Value::Eight});

        let winner = dealer.check_for_winning_hand();
        assert_eq!(winner.len(), 2);
        assert_eq!(winner[0].get_name(), "John");
        assert_eq!(winner[1].get_name(), "Jane");
    }

    #[test]
    fn evaluate_no_winning_hand() {
        let player1 = Player::new("John".to_owned(), 1, 1000);
        let player2 = Player::new("Jane".to_owned(), 2, 1000);
        let player3 = Player::new("Bob".to_owned(), 3, 1000);

        let mut dealer = FiveDrawDealer::new();

        dealer.add_player(player1);
        dealer.add_player(player2);
        dealer.add_player(player3);

        // give player 1 a pair of 2s and 3 random cards
        dealer.players[0].add_card(Card {suit: Suit::Hearts, value: Value::Two});
        dealer.players[0].add_card(Card {suit: Suit::Diamonds, value: Value::Two});
        dealer.players[0].add_card(Card {suit: Suit::Clubs, value: Value::Five});
        dealer.players[0].add_card(Card {suit: Suit::Spades, value: Value::Nine});
        dealer.players[0].add_card(Card {suit: Suit::Hearts, value: Value::King});

        // give player 2 a pair of 2s and 3 random cards
        dealer.players[1].add_card(Card {suit: Suit::Spades, value: Value::Two});
        dealer.players[1].add_card(Card {suit: Suit::Clubs, value: Value::Two});
        dealer.players[1].add_card(Card {suit: Suit::Hearts, value: Value::Five});
        dealer.players[1].add_card(Card {suit: Suit::Diamonds, value: Value::Nine});
        dealer.players[1].add_card(Card {suit: Suit::Spades, value: Value::Queen});

        // give player 3 a pair of 5 random cards
        dealer.players[2].add_card(Card {suit: Suit::Hearts, value: Value::Three});
        dealer.players[2].add_card(Card {suit: Suit::Diamonds, value: Value::Four});
        dealer.players[2].add_card(Card {suit: Suit::Clubs, value: Value::Five});
        dealer.players[2].add_card(Card {suit: Suit::Spades, value: Value::Six});
        dealer.players[2].add_card(Card {suit: Suit::Hearts, value: Value::Eight});

        let winner = dealer.check_for_winning_hand();
        assert_eq!(winner.len(), 1);
        assert_eq!(winner[0].get_name(), "John");
    }

    #[test]
    fn evaluate_royal_flush_vs_straight_flush() {
        let player1 = Player::new("John".to_owned(), 1, 1000);
        let player2 = Player::new("Jane".to_owned(), 2, 1000);
        let player3 = Player::new("Bob".to_owned(), 3, 1000);

        let mut dealer = FiveDrawDealer::new();

        dealer.add_player(player1);
        dealer.add_player(player2);
        dealer.add_player(player3);

        // give player 1 a pair of 2s and 3 random cards
        dealer.players[1].add_card(Card {suit: Suit::Hearts, value: Value::Ten});
        dealer.players[1].add_card(Card {suit: Suit::Hearts, value: Value::Queen});
        dealer.players[1].add_card(Card {suit: Suit::Hearts, value: Value::King});
        dealer.players[1].add_card(Card {suit: Suit::Hearts, value: Value::Jack});
        dealer.players[1].add_card(Card {suit: Suit::Hearts, value: Value::Ace});

        // give player 2 a pair of 2s and 3 random cards
        dealer.players[0].add_card(Card {suit: Suit::Spades, value: Value::King});
        dealer.players[0].add_card(Card {suit: Suit::Spades, value: Value::Ten});
        dealer.players[0].add_card(Card {suit: Suit::Spades, value: Value::Nine});
        dealer.players[0].add_card(Card {suit: Suit::Spades, value: Value::Jack});
        dealer.players[0].add_card(Card {suit: Suit::Spades, value: Value::Queen});

        // give player 3 a pair of 5 random cards
        dealer.players[2].add_card(Card {suit: Suit::Hearts, value: Value::Three});
        dealer.players[2].add_card(Card {suit: Suit::Diamonds, value: Value::Four});
        dealer.players[2].add_card(Card {suit: Suit::Clubs, value: Value::Five});
        dealer.players[2].add_card(Card {suit: Suit::Spades, value: Value::Six});
        dealer.players[2].add_card(Card {suit: Suit::Hearts, value: Value::Seven});

        let winner = dealer.check_for_winning_hand();
        assert_eq!(winner.len(), 1);
        assert_eq!(winner[0].get_name(), "Jane");
    }

}

