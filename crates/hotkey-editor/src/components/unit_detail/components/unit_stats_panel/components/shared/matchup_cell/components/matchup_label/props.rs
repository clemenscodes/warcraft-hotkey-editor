use dioxus::prelude::*;

/// A matchup cell's defense/armor label.
#[derive(Props, Clone, PartialEq)]
pub struct MatchupLabelProps {
    #[props(into)]
    pub text: String,
}
