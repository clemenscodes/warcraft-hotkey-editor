use super::components::matchup_label::MatchupLabelProps;
use super::components::matchup_value::MatchupValueProps;
use super::state::MatchupStrength;
use dioxus::prelude::*;

/// One matchup cell: the defense/attack label, its multiplier value, the tooltip,
/// and how strong the matchup is (which tints the cell and value).
#[derive(Props, Clone, PartialEq)]
pub struct MatchupProps {
    #[props(into)]
    pub label: String,
    #[props(into)]
    pub value: String,
    #[props(into)]
    pub title: String,
    #[props(default)]
    pub strength: MatchupStrength,
}

impl From<&MatchupProps> for MatchupLabelProps {
    fn from(props: &MatchupProps) -> Self {
        let text = props.label.clone();
        Self { text }
    }
}

impl From<&MatchupProps> for MatchupValueProps {
    fn from(props: &MatchupProps) -> Self {
        let text = props.value.clone();
        Self { text }
    }
}
