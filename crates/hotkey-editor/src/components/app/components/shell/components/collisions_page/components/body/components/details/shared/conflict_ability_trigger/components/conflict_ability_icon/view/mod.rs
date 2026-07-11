/// The published `View` contract mirroring [`ConflictAbilityIconModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ConflictAbilityIconView {
    pub src: Option<String>,
    pub alt: String,
}

impl ddd::View for ConflictAbilityIconView {}
