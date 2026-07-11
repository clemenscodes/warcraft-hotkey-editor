use super::view::StrengthRowView;
use dioxus::prelude::*;
use warcraft_keybinds::AttributeStatistic;

/// The strength row's input: the hero's strength at the selected level and whether it
/// is the hero's primary attribute (which glows gold).
#[derive(Props, Clone, PartialEq)]
pub struct StrengthRowModel {
    pub statistic: AttributeStatistic,
    pub is_primary: bool,
}

impl From<&StrengthRowView> for StrengthRowModel {
    fn from(view: &StrengthRowView) -> Self {
        let StrengthRowView {
            statistic,
            is_primary,
        } = view.clone();
        Self {
            statistic,
            is_primary,
        }
    }
}

impl ddd::Model for StrengthRowModel {
    type View = StrengthRowView;
}
