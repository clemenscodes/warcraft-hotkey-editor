use dioxus::prelude::*;

/// A matchup cell's damage multiplier; the leaf renders it as a percentage. Its
/// strong/weak colour comes from the cell.
#[derive(Props, Clone, PartialEq)]
pub struct MatchupValueProps {
    pub multiplier: f32,
}
