use super::view::RangeRowView;
use dioxus::prelude::*;
use warcraft_keybinds::AttackRange;

/// The range row's input: the attack's reach.
#[derive(Props, Clone, PartialEq)]
pub struct RangeRowProps {
    pub value: AttackRange,
}

impl From<&RangeRowView> for RangeRowProps {
    fn from(view: &RangeRowView) -> Self {
        let RangeRowView { value } = view.clone();
        Self { value }
    }
}

impl ddd::Props for RangeRowProps {
    type View = RangeRowView;
}
