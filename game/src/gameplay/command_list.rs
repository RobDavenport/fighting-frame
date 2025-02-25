use std::default;

pub struct CommandList {
    pub moves: &'static [Move],
}

impl CommandList {
    pub fn get_move(&self, input: InputCommand) -> Option<&Move> {
        self.moves.iter().find(|m| m.command == input)
    }
}

#[derive(PartialEq, Eq, Default)]
pub struct DirectionInput {
    pub vertical: VerticalInput,
    pub horizontal: HorizontalInput,
}

#[derive(PartialEq, Eq)]
pub enum ButtonInput {
    A, // Light
    B, // Medium
    C, // Heavy
    S, // Special

    AS, // Throw
    BS, // Boost
    CS, // Super
}

#[derive(PartialEq, Eq, Default)]
pub enum VerticalInput {
    #[default]
    Neutral,
    Up,
    Down,
}

#[derive(PartialEq, Eq, Default)]
pub enum HorizontalInput {
    #[default]
    Neutral,
    Forward,
    Backward,
}

#[derive(PartialEq, Eq)]
pub struct InputCommand {
    pub direction: DirectionInput,
    pub button: ButtonInput,
}

impl DirectionInput {
    /// 7 8 9
    /// 4 5 6
    /// 1 2 3
    pub const fn from_notation(input: &char) -> Self {
        match input {
            '1' => DirectionInput {
                vertical: VerticalInput::Down,
                horizontal: HorizontalInput::Backward,
            },
            '2' => DirectionInput {
                vertical: VerticalInput::Down,
                horizontal: HorizontalInput::Neutral,
            },
            '3' => DirectionInput {
                vertical: VerticalInput::Down,
                horizontal: HorizontalInput::Forward,
            },
            '4' => DirectionInput {
                vertical: VerticalInput::Neutral,
                horizontal: HorizontalInput::Backward,
            },
            '5' => DirectionInput {
                vertical: VerticalInput::Neutral,
                horizontal: HorizontalInput::Neutral,
            },
            '6' => DirectionInput {
                vertical: VerticalInput::Neutral,
                horizontal: HorizontalInput::Forward,
            },
            '7' => DirectionInput {
                vertical: VerticalInput::Up,
                horizontal: HorizontalInput::Backward,
            },
            '8' => DirectionInput {
                vertical: VerticalInput::Up,
                horizontal: HorizontalInput::Neutral,
            },
            '9' => DirectionInput {
                vertical: VerticalInput::Up,
                horizontal: HorizontalInput::Forward,
            },
            _ => panic!(),
        }
    }
}

impl ButtonInput {
    pub const fn from_notation(input: &'static str) -> Self {
        let bytes = input.as_bytes(); // Check if `as_bytes` is const in your Rust version.
        Self::from_notation_bytes(bytes)
    }

    pub const fn from_notation_bytes(bytes: &[u8]) -> Self {
        if bytes.len() == 1 {
            if bytes[0] == b'a' {
                return ButtonInput::A;
            }
            if bytes[0] == b'b' {
                return ButtonInput::B;
            }
            if bytes[0] == b'c' {
                return ButtonInput::C;
            }
            if bytes[0] == b's' {
                return ButtonInput::S;
            }
        } else if bytes.len() == 2 {
            if bytes[0] == b'a' && bytes[1] == b's' {
                return ButtonInput::AS;
            }
            if bytes[0] == b'b' && bytes[1] == b's' {
                return ButtonInput::BS;
            }
            if bytes[0] == b'c' && bytes[1] == b's' {
                return ButtonInput::CS;
            }
        }
        panic!("Invalid input")
    }
}

impl InputCommand {
    pub const fn from_notation(input: &'static str) -> Self {
        let bytes = input.as_bytes();
        let button = match bytes.len() {
            2 => ButtonInput::from_notation_bytes(&[bytes[1]]),
            3 => ButtonInput::from_notation_bytes(&[bytes[1], bytes[2]]),
            _ => panic!("Invalid input."),
        };
        Self {
            direction: DirectionInput::from_notation(&(bytes[0] as char)),
            button,
        }
    }
}

pub enum MoveType {
    Normal,
    Special,
    Super,
}

pub struct Move {
    pub command: InputCommand,
    pub move_type: MoveType,
}
