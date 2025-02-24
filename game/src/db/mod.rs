mod character_definition;

pub use character_definition::*;
use shared::Trs;

pub struct CharacterGraphicsData {
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

impl AnimationData {
    pub fn blend(&self, keyframe: usize, index: usize, s: f32) -> Trs {
        let next = (keyframe + 1) % self.data.len();
        let curr = &self.data[keyframe][index];
        let next = &self.data[next][index];
        curr.lerp(&next, s)
    }
}
