use crate::game_state::GameError;
use crate::game_state::GameError::InvalidTrackIndex;

pub fn get_max_number_of_steps(track_index: usize) -> Result<usize, GameError> {
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
        _ => Err(InvalidTrackIndex(track_index)),
    }
}

// pub fn is_at_top(pos: &TrackPosition) -> Result<bool, GameError> {
//     let track_index = pos.track as usize;
//     let step = pos.step as usize;
//     Ok(step >= get_max_number_of_steps(track_index)?)
// }
