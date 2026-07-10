mod props;
mod view;

pub use view::NeutralMatchupView;
mod style;

use super::shared::matchup_label::MatchupLabel;
use super::shared::matchup_value::MatchupValue;
use dioxus::prelude::*;
use props::NeutralMatchupProps;
use style::CLASS;
use tw_macro::assert_component;

/// The neutral matchup cell: no tint. The value reads its colour from the `--matchup-color` this cell publishes.
#[component]
pub fn NeutralMatchup(props: NeutralMatchupProps) -> Element {
    let subject = props.subject;
    let multiplier = props.multiplier;
    let title = props.title;
    rsx! {
        div {
            class: CLASS,
            title,
            MatchupLabel { subject }
            MatchupValue { multiplier }
        }
    }
}

assert_component!(NeutralMatchup);
