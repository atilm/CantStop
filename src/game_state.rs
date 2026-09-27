use crate::{
    game_state::GameError::{InavlidNumberOfPlayers, InvalidDieNumber, InvalidDiePairs},
    player::{Player, TrackPosition},
};
use thiserror::Error;

const NUMBER_OF_DICE: usize = 4;

#[derive(Error, Debug, PartialEq)]
pub enum GameError {
    #[error("Invalid number of players {0}")]
    InavlidNumberOfPlayers(usize),
    #[error("Invalid die number: {0}")]
    InvalidDieNumber(u32),
    #[error("Invalid die pairs")]
    InvalidDiePairs,
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
                return Err(InvalidDieNumber(v));
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
        first_pair: [usize; 2],
        second_pair: [usize; 2],
        after_move: AfterMove,
    ) -> Result<(), GameError> {
        GameState::check_die_indices(first_pair, second_pair)?;

        let first_pair = first_pair.iter().map(|i| self.dice_values[*i]);
        let second_pair = second_pair.iter().map(|i| self.dice_values[*i]);

        self.players[0]
            .climbers
            .push(TrackPosition::new(first_pair.sum(), 1));
        self.players[0]
            .climbers
            .push(TrackPosition::new(second_pair.sum(), 1));
        Ok(())
    }

    fn check_die_indices(first_pair: [usize; 2], second_pair: [usize; 2]) -> Result<(), GameError> {
        let mut all_indices: Vec<usize> = [&first_pair[..], &second_pair[..]].concat();
        all_indices.sort();

        match all_indices.as_slice() {
            [0, 1, 2, 3] => Ok(()),
            _ => Err(InvalidDiePairs),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game_state::{
        AfterMove::GO_ON,
        GameError::{InavlidNumberOfPlayers, InvalidDieNumber, InvalidDiePairs},
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

        assert_eq!(result_0, InvalidDieNumber(0));
        assert_eq!(result_7, InvalidDieNumber(7));
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

        let active_player = &game.players[0];

        assert_eq!(active_player.climbers.len(), 2);
        assert!(
            active_player
                .climbers
                .iter()
                .any(|c| *c == TrackPosition::new(7, 1)),
        );

        assert!(
            active_player
                .climbers
                .iter()
                .any(|c| *c == TrackPosition::new(2, 1)),
        );
    }

    #[test]
    fn game_move_rejects_invalid_die_indices() {
        let mut game = GameState::new(2).unwrap();
        game.roll_dice([3, 4, 1, 1]).unwrap();

        // Out of range index 4
        assert_eq!(
            game.move_climbers([0, 4], [2, 3], GO_ON).unwrap_err(),
            InvalidDiePairs
        );
        // Double index 1
        assert_eq!(
            game.move_climbers([0, 1], [1, 3], GO_ON).unwrap_err(),
            InvalidDiePairs
        );
    }
}
