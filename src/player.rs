use crate::game_rules;
use thiserror::Error;

#[derive(Debug)]
pub struct TrackPosition {
    track: u32,
    step: u32
}

#[derive(Error, Debug)]
pub enum PlayerError {

}

#[derive(Debug)]
pub struct Player {
    pub base_camps: [Option<u32>; 11],
    pub climbers: [Option<TrackPosition>; 3]
}

impl Player {
    pub fn new() -> Self {
        const NONE_INDEX: Option<u32> = None;
        const NONE_POS: Option<TrackPosition> = None;
        Player { 
            base_camps: [NONE_INDEX; 11],
            climbers: [NONE_POS; 3]
         }
    }

    pub fn advance_climber(&mut self, track_index: usize) -> Result<(), PlayerError> {
        
        Ok(())
    }

    pub fn set_base_camps(&mut self) -> () {
        
    }

    // pub fn has_won(&self) -> bool {
    //     false
    // }
}


#[cfg(test)]
mod tests {
    use crate::player::Player;

    #[test]
    fn player_with_three_base_camps_at_end_has_won() {
        let mut player = Player::new();
    }
}