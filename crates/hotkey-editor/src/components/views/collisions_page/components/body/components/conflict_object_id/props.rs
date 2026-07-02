use dioxus::prelude::*;

/// A unit's database object id, shown as a monospace caption on a collision card.
#[derive(Props, Clone, PartialEq)]
pub struct ConflictObjectIdProps {
    #[props(into)]
    pub text: String,
}
