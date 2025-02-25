use std::collections::VecDeque;

use super::{CommandList, DirectionInput, InputCommand, Move};

const INPUT_BUFFER_LENGTH: usize = 32;

#[derive(Default)]
pub struct FrameCommandState {
    pub frame_count: usize,
    pub state: InputState,
}

impl FrameCommandState {
    pub fn new(state: InputState) -> Self {
        Self {
            frame_count: 0,
            state,
        }
    }
}

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub struct InputState {
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

    pub fn push(&mut self, input_state: InputState) {
        let last = self.buffer.back_mut().unwrap();
        if last.state == input_state {
            last.frame_count += 1;
            return;
        }

        if self.buffer.len() == INPUT_BUFFER_LENGTH {
            self.buffer.pop_front();
        }

        self.buffer.push_back(FrameCommandState::new(input_state));
    }
}
