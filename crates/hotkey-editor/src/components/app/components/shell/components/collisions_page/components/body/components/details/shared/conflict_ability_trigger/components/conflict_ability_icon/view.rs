/// The published `View` contract mirroring [`ConflictAbilityIconProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ConflictAbilityIconView {
    pub src: Option<String>,
    pub alt: String,
}

impl ddd::View for ConflictAbilityIconView {}
