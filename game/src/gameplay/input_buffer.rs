use std::collections::VecDeque;

use super::{CommandList, DirectionInput, InputCommand, Move};

const INPUT_BUFFER_LENGTH: usize = 32;

#[derive(Default)]
pub struct FrameCommandState {
    pub frame_count: usize,
    pub direction: DirectionInput,
    pub a_pressed: bool,
    pub b_pressed: bool,
    pub c_pressed: bool,
    pub s_pressed: bool,
}

pub struct InputBuffer {
    buffer: VecDeque<FrameCommandState>,
}

impl InputBuffer {
    pub fn new() -> Self {
        let mut buffer = VecDeque::with_capacity(INPUT_BUFFER_LENGTH);
        buffer.push_back(FrameCommandState::default());
        Self { buffer }
    }

    pub fn get_last_entry(&self) -> &FrameCommandState {
        self.buffer.back().unwrap()
    }

    pub fn push(&mut self, command_state: FrameCommandState) {
        let last = self.buffer.back_mut().unwrap();
        if last.direction == command_state.direction
            && last.a_pressed == command_state.a_pressed
            && last.b_pressed == command_state.b_pressed
            && last.c_pressed == command_state.c_pressed
            && last.s_pressed == command_state.s_pressed
        {
            last.frame_count += 1;
            return;
        }

        if self.buffer.len() == INPUT_BUFFER_LENGTH {
            self.buffer.pop_front();
        }

        self.buffer.push_back(command_state);
    }
}
