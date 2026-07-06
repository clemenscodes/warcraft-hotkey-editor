pub mod components;
mod props;
mod state;
mod style;

use components::matchup_label::{MatchupLabel, MatchupLabelProps};
use components::matchup_value::{MatchupValue, MatchupValueProps};
use dioxus::prelude::*;
pub use props::MatchupCellProps;
pub use state::MatchupStrength;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(MatchupCell);

/// One matchup cell: label and value, tinted by the matchup strength (the `group`).
#[component]
pub fn MatchupCell(props: MatchupCellProps) -> Element {
    let label = MatchupLabelProps::from(&props);
    let value = MatchupValueProps::from(&props);
    let strength = props.strength.data_attribute();
    let title = props.title;
    rsx! {
        div {
            class: CLASS,
            "data-matchup": strength,
            title,
            MatchupLabel { ..label }
            MatchupValue { ..value }
        }
    }
}
