/// The published `View` contract mirroring [`CardGlowIconProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CardGlowIconView {
    pub source: Option<String>,
    pub alt: String,
}

impl ddd::View for CardGlowIconView {}
