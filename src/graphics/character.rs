use super::MeshId;
use super::animations::Animations;

pub struct Character {
    meshes: &'static [MeshId],
    animations: &'static Animations,
}
