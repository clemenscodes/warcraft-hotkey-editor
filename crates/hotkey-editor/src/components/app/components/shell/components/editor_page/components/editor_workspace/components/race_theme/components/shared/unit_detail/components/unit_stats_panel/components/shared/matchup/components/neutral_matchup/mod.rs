mod props;
mod style;

use super::shared::matchup_label::MatchupLabel;
use super::shared::matchup_value::MatchupValue;
use dioxus::prelude::*;
pub use props::NeutralMatchupProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(NeutralMatchup);

/// The neutral matchup cell: no tint. The value reads its colour from the `--matchup-color` this cell publishes.
#[component]
pub fn NeutralMatchup(props: NeutralMatchupProps) -> Element {
    let label = props.label;
    let value = props.value;
    let title = props.title;
    rsx! {
        div {
            class: CLASS,
            title,
            MatchupLabel { ..label }
            MatchupValue { ..value }
        }
    }
}
