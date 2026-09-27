use crate::game_rules;
use thiserror::Error;

const BASE_CAMPS_PER_PLAYER: usize = 10;
const CLIMBERS_PER_PLAYER: usize = 3;
const NONE_POS: Option<TrackPosition> = None;

#[derive(Debug, PartialEq)]
pub struct TrackPosition {
    track: u32,
    step: u32,
}

impl TrackPosition {
    pub fn new(track: u32, step: u32) -> TrackPosition {
        TrackPosition { track, step }
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
}
