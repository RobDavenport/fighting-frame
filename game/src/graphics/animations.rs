use glam::Mat4;

use crate::gameplay::InputCommand;

pub struct Animations {
    pub data: &'static [AnimationData],
}

#[derive(PartialEq, Eq)]
pub enum AnimationKey {
    IdleStand,
    IdleCrouch,
    WalkForward,
    WalkBackward,
    DashForward,
    DashBackward,
    Jump,
    JumpForward,
    JumpBackward,
    BlockStand,
    BlockCrouch,
    InputCommand(InputCommand),
}

pub struct AnimationData {
    pub animation_key: AnimationKey,
    pub keyframes: &'static [Keyframe],
}

pub struct Keyframe {
    pub duration: usize,
    pub transforms: &'static [Mat4],
}
