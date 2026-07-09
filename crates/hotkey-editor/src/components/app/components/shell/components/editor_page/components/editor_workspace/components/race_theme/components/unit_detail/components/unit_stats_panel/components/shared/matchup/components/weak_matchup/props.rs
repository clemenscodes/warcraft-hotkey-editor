use super::super::shared::matchup_label::MatchupLabelProps;
use super::super::shared::matchup_value::MatchupValueProps;
use dioxus::prelude::*;

/// The weak matchup cell: a danger tint.
#[derive(Props, Clone, PartialEq)]
pub struct WeakMatchupProps {
    pub label: MatchupLabelProps,
    pub value: MatchupValueProps,
    #[props(into)]
    pub title: String,
}
