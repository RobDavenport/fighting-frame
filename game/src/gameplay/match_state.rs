use std::cmp::Ordering;

use crate::db::CharacterDefinition;

use super::{CharacterState, CharacterStats, FrameCommandState, InputBuffer, InputState};

pub struct MatchState {
    pub timer: usize,
    pub player_1: PlayerGameState,
    pub player_2: PlayerGameState,
}

pub struct PlayerGameState {
    pub character: &'static CharacterDefinition,
    pub character_stats: CharacterStats,
    pub win_count: usize,
    pub input_buffer: InputBuffer,
    pub pos_x: isize,
    pub pos_y: isize,

    pub character_state: CharacterState,
    pub freeze_frames: usize,
}

impl PlayerGameState {
    pub fn draw(&self) {
        // TODO: Write this
    }
}

pub enum MatchResult {
    Player1Win,
    Player2Win,
    Draw,
}

impl MatchState {
    pub fn update(&mut self, p1_inputs: InputState, p2_inputs: InputState) -> Option<MatchResult> {
        // Update Player Inputs
        self.player_1.input_buffer.push(p1_inputs);
        self.player_2.input_buffer.push(p2_inputs);

        // TODO: Update the game simulation

        self.update_timer()
    }

    pub fn draw(&self) {
        // TODO: Draw the match state
        // TODO: Draw UI Elements

        self.player_1.draw();
        self.player_2.draw();

        // TODO: Draw Background (Or do this elsewhere?)
    }

    fn update_timer(&mut self) -> Option<MatchResult> {
        if self.timer == 0 {
            let p1_health_percentage =
                self.player_1.character_stats.health as f32 / self.player_1.character.max_hp as f32;
            let p2_health_percentage =
                self.player_2.character_stats.health as f32 / self.player_2.character.max_hp as f32;

            Some(
                match p1_health_percentage
                    .partial_cmp(&p2_health_percentage)
                    .unwrap()
                {
                    Ordering::Less => MatchResult::Player2Win,
                    Ordering::Equal => MatchResult::Draw,
                    Ordering::Greater => MatchResult::Player1Win,
                },
            )
        } else {
            self.timer -= 1;
            None
        }
    }
}
