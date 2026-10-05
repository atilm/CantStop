use crate::{
    game_state::{
        AfterMove::{GoOn, SetCamps},
        GameError::{CannotSetCamp, InavlidNumberOfPlayers, InvalidDieNumber, InvalidDiePairs},
        MoveResult::{GoAgain, NextPlayer},
        SingleMoveResult::{Moved, Passed},
    },
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
    #[error("Invalid track index: {0}")]
    InvalidTrackIndex(usize),
    #[error("Cannot set camps with less than 3 climbers")]
    CannotSetCamp,
}

#[derive(Debug)]
pub struct GameState {
    players: Vec<Player>,
    active_player: usize,
    dice_values: [u32; 4],
}

#[derive(Debug, PartialEq)]
pub enum MoveResult {
    GoAgain,
    NextPlayer,
}

#[derive(PartialEq)]
pub enum SingleMoveResult {
    Moved,
    Passed,
}

pub enum AfterMove {
    GoOn,
    SetCamps,
}

impl GameState {
    pub fn new(players: usize) -> Result<Self, GameError> {
        match players {
            1..5 => {
                let players = (0..players).map(|_| Player::new()).collect();
                let active_player = 0;
                let dice_values = [0; 4];
                Ok(GameState {
                    players,
                    active_player,
                    dice_values,
                })
            }
            _ => Err(InavlidNumberOfPlayers(players)),
        }
    }

    pub fn get_active_player(&self) -> usize {
        self.active_player
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

    // The pairs specify the indices of the dice to combine.
    // Pairs for forbidden moves are ignored.
    // If you are inserting a third climber, but both moves are allowed,
    // the first pair well be preferred and the second ignored.
    // after_move determines, if you want to go on or set camps.
    pub fn move_climbers(
        &mut self,
        first_pair: [usize; 2],
        second_pair: [usize; 2],
        after_move: AfterMove,
    ) -> Result<MoveResult, GameError> {
        GameState::check_die_indices(first_pair, second_pair)?;

        let first_move = self.move_for_pair(first_pair)?;
        let second_move = self.move_for_pair(second_pair)?;

        if first_move == Passed && second_move == Passed {
            self.next_player()
        } else {
            match after_move {
                GoOn => Ok(GoAgain),
                SetCamps => self.set_camps(),
            }
        }
    }

    fn set_camps(&mut self) -> Result<MoveResult, GameError> {
        let player = self.get_current_player_mut();
        if player.climbers.len() < 3 {
            return Err(CannotSetCamp);
        }

        player.base_camps.extend_from_slice(&player.climbers);

        self.next_player()
    }

    fn get_current_player_mut(&mut self) -> &mut Player {
        &mut self.players[self.active_player]
    }

    fn get_current_player(&self) -> &Player {
        &self.players[self.active_player]
    }

    fn next_player(&mut self) -> Result<MoveResult, GameError> {
        let player = self.get_current_player_mut();
        player.climbers.clear();
        self.active_player = (self.active_player + 1) % self.players_count();
        Ok(NextPlayer)
    }

    fn check_die_indices(first_pair: [usize; 2], second_pair: [usize; 2]) -> Result<(), GameError> {
        let mut all_indices: Vec<usize> = [&first_pair[..], &second_pair[..]].concat();
        all_indices.sort();

        match all_indices.as_slice() {
            [0, 1, 2, 3] => Ok(()),
            _ => Err(InvalidDiePairs),
        }
    }

    fn move_for_pair(
        &mut self,
        dice_index_pair: [usize; 2],
    ) -> Result<SingleMoveResult, GameError> {
        let dice_value_pair = dice_index_pair.iter().map(|i| self.dice_values[*i]);
        let track: u32 = dice_value_pair.sum();

        let mut existing_climber = self
            .get_current_player_mut()
            .get_climber_on_track_mut(track);

        match existing_climber {
            Some(ref mut climber) => GameState::maybe_climb(climber),
            None => Ok(self.maybe_add_climber(track)),
        }
    }

    fn maybe_climb(climber: &mut TrackPosition) -> Result<SingleMoveResult, GameError> {
        if climber.is_at_top()? {
            return Ok(Passed);
        }

        climber.step += 1;
        Ok(Moved)
    }

    fn maybe_add_climber(&mut self, track: u32) -> SingleMoveResult {
        if self.get_current_player().climbers.len() >= 3 {
            return Passed;
        }

        let climber_start_position = self
            .get_current_player()
            .get_base_camp_on_track(track)
            .map(|base_camp| base_camp.step + 1)
            .unwrap_or(1);

        self.get_current_player_mut()
            .climbers
            .push(TrackPosition::new(track, climber_start_position));
        Moved
    }
}

#[cfg(test)]
mod tests {
    use crate::game_state::{
        AfterMove::{GoOn, SetCamps},
        GameError::{InavlidNumberOfPlayers, InvalidDieNumber, InvalidDiePairs},
        MoveResult::NextPlayer,
    };

    use super::*;

    fn has_climber(player: &Player, climber: TrackPosition) -> bool {
        player.climbers.iter().any(|c| *c == climber)
    }

    fn has_camp(player: &Player, camp: TrackPosition) -> bool {
        player.base_camps.iter().any(|c| *c == camp)
    }

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
    fn game_move_rejects_invalid_die_indices() {
        let mut game = GameState::new(2).unwrap();
        game.roll_dice([3, 4, 1, 1]).unwrap();

        // Out of range index 4
        assert_eq!(
            game.move_climbers([0, 4], [2, 3], GoOn).unwrap_err(),
            InvalidDiePairs
        );
        // Double index 1
        assert_eq!(
            game.move_climbers([0, 1], [1, 3], GoOn).unwrap_err(),
            InvalidDiePairs
        );
    }

    #[test]
    fn first_player_can_insert_two_climbers() {
        let mut game = GameState::new(2).unwrap();
        game.roll_dice([3, 4, 1, 1]).unwrap();
        game.move_climbers([0, 1], [2, 3], GoOn).unwrap();

        let active_player = &game.get_current_player();

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
    fn first_player_can_insert_one_player_at_step_two() {
        let mut game = GameState::new(2).unwrap();
        game.roll_dice([3, 3, 4, 4]).unwrap();
        game.move_climbers([0, 2], [1, 3], GoOn).unwrap();

        let active_player = &game.get_current_player();

        assert_eq!(active_player.climbers.len(), 1);
        assert!(
            active_player
                .climbers
                .iter()
                .any(|c| *c == TrackPosition::new(7, 2)),
        );
    }

    #[test]
    fn first_player_can_insert_the_third_climber() {
        let mut game = GameState::new(2).unwrap();

        game.roll_dice([3, 3, 4, 4]).unwrap();
        game.move_climbers([0, 1], [2, 3], GoOn).unwrap();

        {
            let active_player = game.get_current_player();
            assert_eq!(active_player.climbers.len(), 2);

            assert!(has_climber(active_player, TrackPosition::new(6, 1)));
            assert!(has_climber(active_player, TrackPosition::new(8, 1)));
        }

        game.roll_dice([5, 1, 5, 2]).unwrap();
        game.move_climbers([0, 1], [2, 3], GoOn).unwrap();

        {
            let active_player = game.get_current_player();
            assert_eq!(active_player.climbers.len(), 3);

            assert!(has_climber(active_player, TrackPosition::new(6, 2)));
            assert!(has_climber(active_player, TrackPosition::new(7, 1)));
            assert!(has_climber(active_player, TrackPosition::new(8, 1)));
        }
    }

    #[test]
    fn no_fourth_climber_is_added() {
        let mut game = GameState::new(2).unwrap();

        game.roll_dice([3, 3, 4, 4]).unwrap();
        game.move_climbers([0, 1], [2, 3], GoOn).unwrap();

        {
            let active_player = game.get_current_player();
            assert_eq!(active_player.climbers.len(), 2);

            assert!(has_climber(active_player, TrackPosition::new(6, 1)));
            assert!(has_climber(active_player, TrackPosition::new(8, 1)));
        }

        game.roll_dice([1, 1, 5, 2]).unwrap();
        game.move_climbers([2, 3], [0, 1], GoOn).unwrap();

        {
            let active_player = game.get_current_player();
            assert_eq!(active_player.climbers.len(), 3);

            assert!(has_climber(active_player, TrackPosition::new(6, 1)));
            assert!(has_climber(active_player, TrackPosition::new(7, 1)));
            assert!(has_climber(active_player, TrackPosition::new(8, 1)));
        }
    }

    fn move_continue(game: &mut GameState, pairs: [u32; 4]) -> MoveResult {
        game.roll_dice(pairs).unwrap();
        game.move_climbers([0, 1], [2, 3], GoOn).unwrap()
    }

    fn move_set_camps(game: &mut GameState, pairs: [u32; 4]) -> MoveResult {
        game.roll_dice(pairs).unwrap();
        game.move_climbers([0, 1], [2, 3], SetCamps).unwrap()
    }

    #[test]
    fn when_a_climber_has_reached_the_top_further_moves_are_ignored() {
        let mut game = GameState::new(2).unwrap();

        assert_eq!(move_continue(&mut game, [1, 1, 6, 6]), GoAgain);
        assert_eq!(move_continue(&mut game, [1, 1, 6, 6]), GoAgain);
        assert_eq!(move_continue(&mut game, [1, 1, 6, 6]), GoAgain);
        assert_eq!(move_continue(&mut game, [1, 1, 2, 6]), GoAgain);
        assert_eq!(move_continue(&mut game, [5, 3, 6, 6]), GoAgain);

        let active_player = game.get_current_player();

        assert!(has_climber(active_player, TrackPosition::new(2, 3)));
        assert!(has_climber(active_player, TrackPosition::new(12, 3)));
        assert!(has_climber(active_player, TrackPosition::new(8, 2)));
    }

    #[test]
    fn when_no_move_is_possible_its_the_next_players_turn() {
        let mut game = GameState::new(2).unwrap();

        assert_eq!(move_continue(&mut game, [1, 1, 6, 6]), GoAgain);
        assert_eq!(move_continue(&mut game, [1, 1, 3, 4]), GoAgain);

        {
            let active_player = game.get_current_player();

            assert!(has_climber(active_player, TrackPosition::new(2, 2)));
            assert!(has_climber(active_player, TrackPosition::new(12, 1)));
            assert!(has_climber(active_player, TrackPosition::new(7, 1)));
        }

        assert_eq!(move_continue(&mut game, [1, 2, 4, 4]), NextPlayer);

        assert_eq!(game.get_active_player(), 1);
    }

    #[test]
    fn a_player_can_set_camps_when_three_climbers_have_been_placed() {
        let mut game = GameState::new(2).unwrap();

        assert_eq!(move_continue(&mut game, [1, 1, 1, 2]), GoAgain);
        assert_eq!(move_set_camps(&mut game, [1, 2, 2, 2]), NextPlayer);

        assert_eq!(game.get_active_player(), 1);

        let previous_player = &game.players[0];
        assert!(has_camp(previous_player, TrackPosition::new(2, 1)));
        assert!(has_camp(previous_player, TrackPosition::new(3, 2)));
        assert!(has_camp(previous_player, TrackPosition::new(4, 1)));

        assert!(previous_player.climbers.is_empty());
    }

    #[test]
    fn cannot_set_camps_when_less_than_three_climbers_have_been_placed() {
        let mut game = GameState::new(2).unwrap();

        assert_eq!(move_continue(&mut game, [1, 1, 1, 2]), GoAgain);

        game.roll_dice([1, 1, 1, 2]).unwrap();
        let error = game.move_climbers([0, 1], [2, 3], SetCamps).unwrap_err();

        assert_eq!(error, GameError::CannotSetCamp)
    }

    #[test]
    fn when_the_last_player_has_played_its_the_first_players_turn_again() {
        let mut game = GameState::new(2).unwrap();
        assert_eq!(game.get_active_player(), 0);

        move_continue(&mut game, [3, 3, 3, 4]);
        move_set_camps(&mut game, [3, 4, 4, 4]);
        assert_eq!(game.get_active_player(), 1);

        move_continue(&mut game, [1, 1, 1, 2]);
        move_set_camps(&mut game, [1, 2, 2, 2]);
        assert_eq!(game.get_active_player(), 0);
    }

    #[test]
    fn climbers_are_inserted_after_existing_base_camps() {
        let mut game = GameState::new(2).unwrap();

        // Player 0 sets camps on tracks 6, 7, 8
        move_continue(&mut game, [3, 3, 3, 4]);
        move_continue(&mut game, [3, 3, 3, 4]);
        move_set_camps(&mut game, [3, 4, 4, 4]);

        // Player 1
        move_continue(&mut game, [1, 1, 1, 2]);
        move_set_camps(&mut game, [1, 2, 2, 2]);

        // Player 0 reinserts climbers on tracks 6, 7, 8
        move_continue(&mut game, [3, 3, 3, 4]);
        move_continue(&mut game, [4, 4, 5, 4]);

        let player = game.get_current_player();
        assert!(has_camp(player, TrackPosition::new(6, 2)));
        assert!(has_camp(player, TrackPosition::new(7, 3)));
        assert!(has_camp(player, TrackPosition::new(8, 1)));

        assert!(has_climber(player, TrackPosition::new(6, 3)));
        assert!(has_climber(player, TrackPosition::new(7, 4)));
        assert!(has_climber(player, TrackPosition::new(8, 2)));
    }

    #[test]
    fn when_a_track_is_closed_other_players_base_camps_are_removed() {
        assert!(false)
    }
}
