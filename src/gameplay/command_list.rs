pub struct CommandList {
    moves: &'static [Move],
}

impl CommandList {
    pub fn get_move(&self, input: InputCommand) -> Option<&Move> {
        self.moves.iter().find(|m| m.command == input)
    }
}

#[derive(PartialEq, Eq)]
pub struct DirectionInput {
    vertical: VerticalInput,
    horizontal: VerticalInput,
}

#[derive(PartialEq, Eq)]
pub enum Button {
    A, // Light
    B, // Medium
    C, // Heavy
    S, // Special

    AS, // Throw
    BS, // Boost
    CS, // Super
}

#[derive(PartialEq, Eq)]
pub enum VerticalInput {
    Neutral,
    Up,
    Down,
}

#[derive(PartialEq, Eq)]
pub enum HorizontalInput {
    Neutral,
    Forward,
    Backward,
}

#[derive(PartialEq, Eq)]
pub struct InputCommand {
    direction: DirectionInput,
    button: Button,
}

pub enum MoveType {
    Normal,
    Special,
    Super,
}

pub struct Move {
    command: InputCommand,
    move_type: MoveType,
}
