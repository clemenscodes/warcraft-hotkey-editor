#[derive(Clone, PartialEq)]
pub struct ConflictAbilityIconView {
    pub src: Option<String>,
    pub alt: String,
}

impl ddd::View for ConflictAbilityIconView {}
