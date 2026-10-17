pub mod components;
mod model;
mod view;

pub use view::StrongMatchupHostView;
mod style;

use components::strong_matchup::StrongMatchup;
use dioxus::prelude::*;
use model::StrongMatchupHostModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn StrongMatchupHost(props: StrongMatchupHostModel) -> Element {
    let subject = props.subject;
    let multiplier = props.multiplier;
    let title = props.title;
    rsx! {
        div {
            class: CLASS,
            StrongMatchup {
                subject,
                multiplier,
                title,
            }
        }
    }
}

assert_component!(StrongMatchupHost);
