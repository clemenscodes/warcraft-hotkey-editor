#[derive(Clone, PartialEq)]
pub struct ConflictAbilityIconView {
    pub src: Option<String>,
    pub alt: String,
}

impl ddd::Layered for ConflictAbilityIconView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ConflictAbilityIconView {}
