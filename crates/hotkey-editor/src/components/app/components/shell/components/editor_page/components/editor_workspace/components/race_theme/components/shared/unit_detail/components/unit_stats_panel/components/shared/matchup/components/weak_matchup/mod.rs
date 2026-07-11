mod model;
mod view;

pub use view::WeakMatchupView;
mod style;

use super::shared::matchup_label::MatchupLabel;
use super::shared::matchup_value::MatchupValue;
use dioxus::prelude::*;
use model::WeakMatchupModel;
use style::CLASS;
use tw_macro::assert_component;

/// The weak matchup cell: a danger tint. The value reads its colour from the `--matchup-color` this cell publishes.
#[component]
pub fn WeakMatchup(props: WeakMatchupModel) -> Element {
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

assert_component!(WeakMatchup);
