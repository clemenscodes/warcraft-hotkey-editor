use super::view::AttackTypeRowView;
use dioxus::prelude::*;
use warcraft_api::AttackType;

/// The attack type row's input: the unit's attack classification.
#[derive(Props, Clone, PartialEq)]
pub struct AttackTypeRowProps {
    pub value: AttackType,
}

impl From<&AttackTypeRowView> for AttackTypeRowProps {
    fn from(view: &AttackTypeRowView) -> Self {
        let AttackTypeRowView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Props for AttackTypeRowProps {
    type View = AttackTypeRowView;
}
