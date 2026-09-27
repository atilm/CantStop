use crate::game_rules;
use thiserror::Error;

const BASE_CAMPS_PER_PLAYER: usize = 10;
const CLIMBERS_PER_PLAYER: usize = 3;
const NONE_POS: Option<TrackPosition> = None;

#[derive(Debug, PartialEq)]
pub struct TrackPosition {
    pub track: u32,
    pub step: u32,
}

impl TrackPosition {
    pub fn new(track: u32, step: u32) -> TrackPosition {
        TrackPosition { track, step }
    }
}

#[derive(Debug)]
pub struct Player {
    pub base_camps: [Option<TrackPosition>; BASE_CAMPS_PER_PLAYER],
    pub climbers: [Option<TrackPosition>; CLIMBERS_PER_PLAYER],
}

impl Player {
    pub fn new() -> Self {
        Player {
            base_camps: [NONE_POS; BASE_CAMPS_PER_PLAYER],
            climbers: [NONE_POS; CLIMBERS_PER_PLAYER],
        }
    }
}
