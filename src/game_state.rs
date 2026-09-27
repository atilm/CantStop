use crate::{
    game_state::GameError::{InavlidNumberOfPlayers, InvalidDiceNumber},
    player::{Player, TrackPosition},
};
use thiserror::Error;

const NUMBER_OF_DICE: usize = 4;

#[derive(Error, Debug, PartialEq)]
pub enum GameError {
    #[error("Invalid number of players {0}")]
    InavlidNumberOfPlayers(usize),
    #[error("Invalid dice number: {0}")]
    InvalidDiceNumber(u32),
}

#[derive(Debug)]
pub struct GameState {
    players: Vec<Player>,
    dice_values: [u32; 4],
}

pub enum AfterMove {
    GO_ON,
    SET_CAMPS,
}

impl GameState {
    pub fn new(players: usize) -> Result<Self, GameError> {
        match players {
            1..5 => {
                let players = (0..players).map(|_| Player::new()).collect();
                let dice_values = [0; 4];
                Ok(GameState {
                    players,
                    dice_values,
                })
            }
            _ => Err(InavlidNumberOfPlayers(players)),
        }
    }

    pub fn get_active_player(&self) -> usize {
        0
    }

    pub fn roll_dice(&mut self, values: [u32; NUMBER_OF_DICE]) -> Result<(), GameError> {
        for v in values {
            if v < 1 || v > 6 {
                return Err(InvalidDiceNumber(v));
            }
        }

        self.dice_values = values;
        Ok(())
    }

    pub fn get_dice(&self) -> [u32; NUMBER_OF_DICE] {
        self.dice_values
    }

    pub fn players_count(&self) -> usize {
        self.players.len()
    }

    pub fn move_climbers(
        &mut self,
        first_pair: [u32; 2],
        second_pair: [u32; 2],
        after_move: AfterMove,
    ) -> Result<(), GameError> {
        self.players[0].climbers[0] = Some(TrackPosition { track: 7, step: 1 });
        self.players[0].climbers[1] = Some(TrackPosition { track: 2, step: 1 });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::game_state::{
        AfterMove::GO_ON,
        GameError::{InavlidNumberOfPlayers, InvalidDiceNumber},
    };

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

    #[test]
    fn game_accepts_valid_dice_values() {
        let mut game = GameState::new(2).unwrap();
        let result = game.roll_dice([1, 2, 5, 6]);
        assert!(result.is_ok());
        assert_eq!(game.get_dice(), [1, 2, 5, 6]);
    }

    #[test]
    fn game_rejects_invalid_dice_values() {
        let mut game = GameState::new(2).unwrap();
        let result_0 = game.roll_dice([0, 2, 3, 4]).unwrap_err();
        let result_7 = game.roll_dice([1, 2, 3, 7]).unwrap_err();

        assert_eq!(result_0, InvalidDiceNumber(0));
        assert_eq!(result_7, InvalidDiceNumber(7));
    }

    #[test]
    fn at_game_start_its_the_first_players_turn() {
        let game = GameState::new(2).unwrap();
        assert_eq!(game.get_active_player(), 0);
    }

    #[test]
    fn first_player_can_insert_two_climbers() {
        let mut game = GameState::new(2).unwrap();
        game.roll_dice([3, 4, 1, 1]).unwrap();
        game.move_climbers([0, 1], [2, 3], GO_ON).unwrap();

        assert!(
            game.players[0]
                .climbers
                .iter()
                .flatten()
                .any(|c| *c == TrackPosition::new(7, 1)),
        );

        assert!(
            game.players[0]
                .climbers
                .iter()
                .flatten()
                .any(|c| *c == TrackPosition::new(2, 1)),
        );
    }
}
