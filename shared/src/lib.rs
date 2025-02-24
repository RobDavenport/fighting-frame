use glam::{Mat4, Quat, Vec3};

#[derive(Clone)]
pub struct Trs {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl From<Mat4> for Trs {
    fn from(value: Mat4) -> Self {
        let (scale, rotation, translation) = value.to_scale_rotation_translation();
        Self {
            translation,
            rotation,
            scale,
        }
    }
}

impl Trs {
    pub fn matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }

    pub fn lerp(&self, other: &Self, s: f32) -> Self {
        Self {
            translation: self.translation.lerp(other.translation, s),
            rotation: self.rotation.slerp(other.rotation, s),
            scale: self.scale.lerp(other.scale, s),
        }
    }
}
