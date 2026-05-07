pub(crate) struct CursorPoint {
    x: f64,
    y: f64,
}

impl CursorPoint {
    pub(crate) fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

pub(crate) struct HitTestPoint {
    x: f32,
    y: f32,
}

impl From<CursorPoint> for HitTestPoint {
    fn from(cursor: CursorPoint) -> Self {
        let x = cursor.x as f32;
        let y = cursor.y as f32;
        Self { x, y }
    }
}

impl HitTestPoint {
    pub(crate) fn x(&self) -> f32 {
        self.x
    }

    pub(crate) fn y(&self) -> f32 {
        self.y
    }
}
