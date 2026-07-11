use super::view::DamagePerSecondRowView;
use dioxus::prelude::*;
use warcraft_api::DamagePerSecond;

/// The damage-per-second row's input: the derived rate, or `None` when the attack has
/// no real cooldown (so a rate is undefined and the row is absent).
#[derive(Props, Clone, PartialEq)]
pub struct DamagePerSecondRowModel {
    pub value: Option<DamagePerSecond>,
}

impl From<&DamagePerSecondRowView> for DamagePerSecondRowModel {
    fn from(view: &DamagePerSecondRowView) -> Self {
        let DamagePerSecondRowView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Model for DamagePerSecondRowModel {
    type View = DamagePerSecondRowView;
}
