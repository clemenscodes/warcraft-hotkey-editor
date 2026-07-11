/// The published `View` contract mirroring [`CardGlowIconModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CardGlowIconView {
    pub source: Option<String>,
    pub alt: String,
}

impl ddd::View for CardGlowIconView {}
