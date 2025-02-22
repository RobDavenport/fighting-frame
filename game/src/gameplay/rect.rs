use glam::IVec2;

pub struct Rect {
    pub top_left: IVec2,
    pub bottom_right: IVec2,
}

impl Rect {
    pub fn collides(&self, other: &Rect) -> bool {
        let horizontal_overlap =
            self.top_left.x < other.bottom_right.x && self.bottom_right.x > other.top_left.x;

        let vertical_overlap =
            self.top_left.y < other.bottom_right.y && self.bottom_right.y > other.top_left.y;

        horizontal_overlap && vertical_overlap
    }
}
