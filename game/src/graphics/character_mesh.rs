use super::{MeshId, animations::Animations};

pub struct CharacterMesh {
    pub meshes: &'static [MeshId],
    pub animations: &'static Animations,
}
