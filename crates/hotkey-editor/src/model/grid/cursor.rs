pub(crate) struct CursorPoint {
    horizontal_position: f64,
    vertical_position: f64,
}

impl CursorPoint {
    pub(crate) fn new(horizontal_position: f64, vertical_position: f64) -> Self {
        Self {
            horizontal_position,
            vertical_position,
        }
    }
}

pub(crate) struct HitTestPoint {
    horizontal_position: f32,
    vertical_position: f32,
}

impl From<CursorPoint> for HitTestPoint {
    fn from(cursor: CursorPoint) -> Self {
        let horizontal_position = cursor.horizontal_position as f32;
        let vertical_position = cursor.vertical_position as f32;
        Self {
            horizontal_position,
            vertical_position,
        }
    }
}

impl HitTestPoint {
    pub(crate) fn horizontal_position(&self) -> f32 {
        self.horizontal_position
    }

    pub(crate) fn vertical_position(&self) -> f32 {
        self.vertical_position
    }
}
