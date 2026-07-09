use dioxus::prelude::*;
use warcraft_keybinds::AttributeStatistic;

/// The agility row's input: the hero's agility at the selected level and whether it
/// is the hero's primary attribute (which glows gold).
#[derive(Props, Clone, PartialEq)]
pub struct AgilityRowProps {
    pub statistic: AttributeStatistic,
    pub is_primary: bool,
}
