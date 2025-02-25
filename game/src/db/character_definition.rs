use crate::{
    gameplay::{CommandList, InputCommand, Move, MoveType},
    graphics::{AnimationData, AnimationKey, Animations},
    static_data,
};

use super::graphics::CharacterGraphicsData;

pub struct CharacterDefinition {
    pub name: &'static str,
    pub max_hp: usize,
    pub forward_walk_speed: isize,
    pub backward_walk_speed: isize,

    pub command_list: &'static CommandList,
    pub graphics: &'static CharacterGraphicsData,
}

pub static DEFAULT_CHARACTER: CharacterDefinition = CharacterDefinition {
    name: "Default Character",
    max_hp: 1000,
    forward_walk_speed: 15,
    backward_walk_speed: 12,
    command_list: &DEFAULT_CHARACTER_COMMAND_LIST,
    graphics: &static_data::CHARACTER_GRAPHICS_DATA,
};

static DEFAULT_CHARACTER_COMMAND_LIST: CommandList = CommandList {
    moves: &[
        Move {
            command: InputCommand::from_notation("5a"),
            move_type: MoveType::Normal,
        },
        Move {
            command: InputCommand::from_notation("5b"),
            move_type: MoveType::Normal,
        },
        Move {
            command: InputCommand::from_notation("5c"),
            move_type: MoveType::Normal,
        },
        Move {
            command: InputCommand::from_notation("2a"),
            move_type: MoveType::Normal,
        },
        Move {
            command: InputCommand::from_notation("2b"),
            move_type: MoveType::Normal,
        },
        Move {
            command: InputCommand::from_notation("2c"),
            move_type: MoveType::Normal,
        },
        Move {
            command: InputCommand::from_notation("4s"), // Parry
            move_type: MoveType::Special,
        },
        Move {
            command: InputCommand::from_notation("5s"), // Upper
            move_type: MoveType::Special,
        },
        Move {
            command: InputCommand::from_notation("6s"), // Strong Upper
            move_type: MoveType::Special,
        },
        Move {
            command: InputCommand::from_notation("1s"), // Slow Fb
            move_type: MoveType::Special,
        },
        Move {
            command: InputCommand::from_notation("2s"), // Medium Fb
            move_type: MoveType::Special,
        },
        Move {
            command: InputCommand::from_notation("3s"), // Fast FB
            move_type: MoveType::Special,
        },
    ],
};
