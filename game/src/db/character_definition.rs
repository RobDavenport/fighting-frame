use crate::{
    gameplay::{CommandList, InputCommand, Move, MoveType},
    graphics::{AnimationData, AnimationKey, Animations, CharacterMesh, MeshId},
};

pub struct CharacterDefinition {
    pub name: &'static str,
    pub max_hp: usize,

    pub command_list: &'static CommandList,
    pub mesh: &'static CharacterMesh,
}

pub const DEFAULT_CHARACTER: &'static CharacterDefinition = &CharacterDefinition {
    name: "Default Character",
    max_hp: 1000,
    command_list: DEFAULT_CHARACTER_COMMAND_LIST,
    mesh: DEFAULT_CHARACTER_MESH,
};

const DEFAULT_CHARACTER_MESH: &'static CharacterMesh = &CharacterMesh {
    meshes: &[],
    animations: DEFAULT_CHARACTER_ANIMATIONS,
};

const DEFAULT_CHARACTER_ANIMATIONS: &'static Animations = &Animations {
    data: &[
        AnimationData {
            animation_key: AnimationKey::IdleStand,
            keyframes: &[],
        },
        AnimationData {
            animation_key: AnimationKey::InputCommand(InputCommand::from_notation("5s")),
            keyframes: &[],
        },
    ],
};

const DEFAULT_CHARACTER_COMMAND_LIST: &'static CommandList = &CommandList {
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
