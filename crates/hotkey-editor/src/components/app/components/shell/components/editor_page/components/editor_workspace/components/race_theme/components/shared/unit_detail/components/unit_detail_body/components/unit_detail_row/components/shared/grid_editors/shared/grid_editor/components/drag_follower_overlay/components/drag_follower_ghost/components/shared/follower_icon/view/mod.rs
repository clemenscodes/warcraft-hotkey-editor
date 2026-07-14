#[derive(Clone, PartialEq)]
pub struct FollowerIconView {
    pub src: String,
    pub alt: String,
}

impl ddd::View for FollowerIconView {}
