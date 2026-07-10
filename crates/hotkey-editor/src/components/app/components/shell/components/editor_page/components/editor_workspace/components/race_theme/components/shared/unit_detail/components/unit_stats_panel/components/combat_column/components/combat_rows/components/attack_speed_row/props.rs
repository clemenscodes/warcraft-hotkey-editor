use super::view::AttackSpeedRowView;
use dioxus::prelude::*;
use warcraft_keybinds::AttackSpeed;

/// The attack speed row's input: the unit's attack cooldown.
#[derive(Props, Clone, PartialEq)]
pub struct AttackSpeedRowProps {
    pub value: AttackSpeed,
}

impl From<&AttackSpeedRowView> for AttackSpeedRowProps {
    fn from(view: &AttackSpeedRowView) -> Self {
        let AttackSpeedRowView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Props for AttackSpeedRowProps {
    type View = AttackSpeedRowView;
}
