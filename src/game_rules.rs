use thiserror::Error;

use crate::game_rules::GameRulesError::InvalidTrackIndex;

#[derive(Error, Debug)]
pub enum GameRulesError {
    #[error("Invalid track index")]
    InvalidTrackIndex,
}

pub fn get_max_number_of_steps(track_index: usize) -> Result<usize, GameRulesError> {
    match track_index {
        2 => Ok(3),
        3 => Ok(5),
        4 => Ok(7),
        5 => Ok(9),
        6 => Ok(11),
        7 => Ok(13),
        8 => Ok(11),
        9 => Ok(9),
        10 => Ok(7),
        11 => Ok(5),
        12 => Ok(3),
        _ => Err(InvalidTrackIndex),
    }
}
