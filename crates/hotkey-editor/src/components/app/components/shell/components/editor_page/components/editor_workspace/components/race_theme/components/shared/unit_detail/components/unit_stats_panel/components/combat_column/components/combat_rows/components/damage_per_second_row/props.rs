use super::view::DamagePerSecondRowView;
use dioxus::prelude::*;
use warcraft_keybinds::DamagePerSecond;

/// The damage-per-second row's input: the derived rate, or `None` when the attack has
/// no real cooldown (so a rate is undefined and the row is absent).
#[derive(Props, Clone, PartialEq)]
pub struct DamagePerSecondRowProps {
    pub value: Option<DamagePerSecond>,
}

impl From<&DamagePerSecondRowView> for DamagePerSecondRowProps {
    fn from(view: &DamagePerSecondRowView) -> Self {
        let DamagePerSecondRowView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Props for DamagePerSecondRowProps {
    type View = DamagePerSecondRowView;
}
