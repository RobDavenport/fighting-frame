use super::Move;

pub enum CharacterState {
    Idle,
    Walking,
    Jumping,
    Attacking(&'static Move),
    Blockstun,
    Hitstun(usize),
    KnockedDown(usize),
    Juggle,
}
