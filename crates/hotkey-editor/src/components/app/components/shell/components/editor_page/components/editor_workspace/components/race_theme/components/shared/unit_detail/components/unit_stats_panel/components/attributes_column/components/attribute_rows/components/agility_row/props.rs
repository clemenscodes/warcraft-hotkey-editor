use super::view::AgilityRowView;
use dioxus::prelude::*;
use warcraft_keybinds::AttributeStatistic;

/// The agility row's input: the hero's agility at the selected level and whether it
/// is the hero's primary attribute (which glows gold).
#[derive(Props, Clone, PartialEq)]
pub struct AgilityRowProps {
    pub statistic: AttributeStatistic,
    pub is_primary: bool,
}

impl From<&AgilityRowView> for AgilityRowProps {
    fn from(view: &AgilityRowView) -> Self {
        let AgilityRowView {
            statistic,
            is_primary,
        } = view.clone();
        Self {
            statistic,
            is_primary,
        }
    }
}

impl ddd::Props for AgilityRowProps {
    type View = AgilityRowView;
}
