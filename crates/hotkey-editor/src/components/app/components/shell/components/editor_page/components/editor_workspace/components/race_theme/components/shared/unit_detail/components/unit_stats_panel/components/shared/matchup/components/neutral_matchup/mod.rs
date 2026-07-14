mod model;
mod view;

pub use view::NeutralMatchupView;
mod style;

use super::shared::matchup_label::MatchupLabel;
use super::shared::matchup_value::MatchupValue;
use dioxus::prelude::*;
use model::NeutralMatchupModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn NeutralMatchup(props: NeutralMatchupModel) -> Element {
    let subject = props.subject;
    let multiplier = props.multiplier;
    let title = props.title;
    rsx! {
        div {
            class: CLASS,
            title,
            MatchupLabel {
                subject,
            }
            MatchupValue {
                multiplier,
            }
        }
    }
}

assert_component!(NeutralMatchup);
