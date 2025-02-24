mod character_definition;

pub use character_definition::*;
use shared::Trs;

pub struct CharacterData {
    pub meshes: &'static [MeshData],
    pub animations: &'static [AnimationData],
}

pub struct MeshData {
    pub vertices: &'static [f32],
    pub indices: &'static [u16],
}

pub struct AnimationData {
    pub name: &'static str,
    pub data: &'static [&'static [Trs]],
}
