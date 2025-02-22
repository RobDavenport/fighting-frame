use glam::Mat4;

pub struct Animations {
    data: &'static [AnimationData],
}

#[derive(PartialEq, Eq)]
pub struct AnimationKey {}

pub struct AnimationData {
    pub animation_key: AnimationKey,
    pub transforms: &'static [Mat4],
}
