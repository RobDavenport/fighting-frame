use crate::db::CharacterDefinition;

use super::{CharacterState, CharacterStats, InputBuffer};

pub struct MatchState {
    pub timer: f32,
    pub player_1: PlayerGameState,
    pub player_2: PlayerGameState,
}

pub struct PlayerGameState {
    pub character: &'static CharacterDefinition,
    pub stats: CharacterStats,
    pub win_count: usize,
    pub input_buffer: InputBuffer,
    pub pos_x: isize,
    pub pos_y: isize,

    pub character_state: CharacterState,
    pub freeze_frames: usize,
}
