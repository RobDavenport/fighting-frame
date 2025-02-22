use super::{collision_box::CollisionBoxes, command_list::MoveType};

pub struct MoveData {
    move_type: MoveType,
    frame_data: &'static [FrameData],
}

impl MoveData {
    pub fn get_total_frames(&self) -> usize {
        self.frame_data.iter().map(|f| f.frame_duration).sum()
    }
}

pub enum CancelProperties {
    NoCancel,
    SpecialCancel,
}

pub enum AttackData {
    Strike(StrikeData),
    Throw(ThrowData),
}

pub enum ThrowData {
    // TODO
}

pub struct StrikeData {
    damage: usize,
    hit_stun: usize,
    block_stun: usize,
    block_hitstop_frames: usize,
    hit_hitstop_frames: usize,
    cancel_properties: CancelProperties,
    kit_kind: HitKind,
}

pub enum HitKind {
    High,
    Medium,
    Low,
}

pub struct Hitboxes {
    attack_data: AttackData,
    collision: &'static [CollisionBoxes],
}

pub struct Hurtboxes {
    collision: &'static [CollisionBoxes],
}

pub struct FrameData {
    frame_duration: usize,
    hitboxes: Option<Hitboxes>,
    hurtboxes: Option<Hurtboxes>,
    blocking_volume: &'static [CollisionBoxes],
}
