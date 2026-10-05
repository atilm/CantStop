use crate::game_rules::get_max_number_of_steps;
use crate::game_state::GameError;

const BASE_CAMPS_PER_PLAYER: usize = 10;
const CLIMBERS_PER_PLAYER: usize = 3;
const NONE_POS: Option<TrackPosition> = None;

#[derive(Debug, PartialEq, Clone)]
pub struct TrackPosition {
    pub track: u32,
    pub step: u32,
}

impl TrackPosition {
    pub fn new(track: u32, step: u32) -> TrackPosition {
        TrackPosition { track, step }
    }

    pub fn is_on_track(&self, track: u32) -> bool {
        self.track == track
    }

    pub fn is_at_top(&self) -> Result<bool, GameError> {
        let track_index = self.track as usize;
        let step = self.step as usize;
        Ok(step >= get_max_number_of_steps(track_index)?)
    }
}

#[derive(Debug)]
pub struct Player {
    pub base_camps: Vec<TrackPosition>,
    pub climbers: Vec<TrackPosition>,
}

impl Player {
    pub fn new() -> Self {
        Player {
            base_camps: Vec::with_capacity(BASE_CAMPS_PER_PLAYER),
            climbers: Vec::with_capacity(CLIMBERS_PER_PLAYER),
        }
    }

    pub fn get_base_camp_on_track(&self, track: u32) -> Option<&TrackPosition> {
        self.base_camps.iter().find(|&c| c.is_on_track(track))
    }

    pub fn get_climber_on_track_mut(&mut self, track: u32) -> Option<&mut TrackPosition> {
        self.climbers
            .iter_mut()
            .find(|ref mut c| c.is_on_track(track))
    }
}
