use dioxus::prelude::*;

/// A matchup cell's multiplier value; its strong/weak colour comes from the cell.
#[derive(Props, Clone, PartialEq)]
pub struct MatchupValueProps {
    #[props(into)]
    pub text: String,
}
