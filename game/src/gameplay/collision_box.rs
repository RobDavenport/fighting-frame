use super::rect::Rect;

pub struct CollisionBoxes {
    data: &'static [Rect],
}

impl CollisionBoxes {
    pub fn collides(&self, other: &CollisionBoxes) -> bool {
        for rect in self.data.iter() {
            for target in other.data.iter() {
                if rect.collides(target) {
                    return true;
                }
            }
        }

        false
    }
}
