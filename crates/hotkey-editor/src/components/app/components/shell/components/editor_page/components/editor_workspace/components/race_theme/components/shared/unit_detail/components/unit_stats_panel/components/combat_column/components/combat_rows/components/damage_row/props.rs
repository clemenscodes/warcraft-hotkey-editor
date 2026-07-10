use super::view::DamageRowView;
use dioxus::prelude::*;
use warcraft_keybinds::DamageRange;

/// The damage row's input: the unit's attack damage range.
#[derive(Props, Clone, PartialEq)]
pub struct DamageRowProps {
    pub value: DamageRange,
}

impl From<&DamageRowView> for DamageRowProps {
    fn from(view: &DamageRowView) -> Self {
        let DamageRowView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Props for DamageRowProps {
    type View = DamageRowView;
}
