/// The published `View` contract mirroring [`FightIconProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct FightIconView {
    pub src: Option<String>,
    pub alt: String,
}

impl ddd::View for FightIconView {}
