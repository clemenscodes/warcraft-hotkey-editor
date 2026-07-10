use super::view::IntelligenceRowView;
use dioxus::prelude::*;
use warcraft_keybinds::AttributeStatistic;

/// The intelligence row's input: the hero's intelligence at the selected level and whether it
/// is the hero's primary attribute (which glows gold).
#[derive(Props, Clone, PartialEq)]
pub struct IntelligenceRowProps {
    pub statistic: AttributeStatistic,
    pub is_primary: bool,
}

impl From<&IntelligenceRowView> for IntelligenceRowProps {
    fn from(view: &IntelligenceRowView) -> Self {
        let IntelligenceRowView {
            statistic,
            is_primary,
        } = view.clone();
        Self {
            statistic,
            is_primary,
        }
    }
}

impl ddd::Props for IntelligenceRowProps {
    type View = IntelligenceRowView;
}
