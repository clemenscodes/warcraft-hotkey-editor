#[derive(Clone, PartialEq)]
pub struct CollisionCountView {
    pub count: usize,
}

impl ddd::View for CollisionCountView {}
