pub mod components;
mod model;
mod view;

pub use view::NeutralMatchupHostView;
mod style;

use components::neutral_matchup::NeutralMatchup;
use dioxus::prelude::*;
use model::NeutralMatchupHostModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn NeutralMatchupHost(props: NeutralMatchupHostModel) -> Element {
    let subject = props.subject;
    let multiplier = props.multiplier;
    let title = props.title;
    rsx! {
        div {
            class: CLASS,
            NeutralMatchup {
                subject,
                multiplier,
                title,
            }
        }
    }
}

assert_component!(NeutralMatchupHost);
