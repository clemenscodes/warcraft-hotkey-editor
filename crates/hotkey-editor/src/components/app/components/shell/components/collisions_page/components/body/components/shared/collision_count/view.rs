/// The published `View` contract mirroring [`CollisionCountProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CollisionCountView {
    pub count: usize,
}

impl ddd::View for CollisionCountView {}
