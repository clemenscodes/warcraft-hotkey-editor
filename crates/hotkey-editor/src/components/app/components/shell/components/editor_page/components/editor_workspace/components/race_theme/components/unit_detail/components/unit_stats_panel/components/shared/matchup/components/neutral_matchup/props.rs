use super::super::shared::matchup_label::MatchupLabelProps;
use super::super::shared::matchup_value::MatchupValueProps;
use dioxus::prelude::*;

/// The neutral matchup cell: no tint.
#[derive(Props, Clone, PartialEq)]
pub struct NeutralMatchupProps {
    pub label: MatchupLabelProps,
    pub value: MatchupValueProps,
    #[props(into)]
    pub title: String,
}
