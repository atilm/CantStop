use crate::{game_state::GameError::InavlidNumberOfPlayers, player::Player};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum GameError {
    #[error("Invalid number of players {0}")]
    InavlidNumberOfPlayers(usize),
}

#[derive(Debug)]
pub struct GameState {
    players: Vec<Player>,
}

impl GameState {
    pub fn new(players: usize) -> Result<Self, GameError> {
        match players {
            1..5 => {
                let players = (0..players).map(|_| Player::new()).collect();
                Ok(GameState { players })
            },
            _ => Err(InavlidNumberOfPlayers(players))
        }
    }

    pub fn players_count(&self) -> usize {
        self.players.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::game_state::GameError::InavlidNumberOfPlayers;

    use super::*;

    #[test]
    fn game_state_returns_player_count() {
        let game = GameState::new(3).unwrap();
        assert_eq!(3, game.players_count())
    }

    #[test]
    fn only_1_to_4_players_are_allowed() {
        assert_eq!(GameState::new(0).unwrap_err(), InavlidNumberOfPlayers(0));
        assert!(GameState::new(1).is_ok());
        assert!(GameState::new(2).is_ok());
        assert!(GameState::new(3).is_ok());
        assert!(GameState::new(4).is_ok());
        assert_eq!(GameState::new(5).unwrap_err(), InavlidNumberOfPlayers(5));
    }
}
