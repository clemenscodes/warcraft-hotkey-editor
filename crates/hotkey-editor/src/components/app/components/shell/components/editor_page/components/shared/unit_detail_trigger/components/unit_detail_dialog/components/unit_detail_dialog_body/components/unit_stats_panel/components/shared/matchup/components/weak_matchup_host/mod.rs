pub mod components;
mod model;
mod view;

pub use view::WeakMatchupHostView;
mod style;

use components::weak_matchup::WeakMatchup;
use dioxus::prelude::*;
use model::WeakMatchupHostModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn WeakMatchupHost(props: WeakMatchupHostModel) -> Element {
    let subject = props.subject;
    let multiplier = props.multiplier;
    let title = props.title;
    rsx! {
        div {
            class: CLASS,
            WeakMatchup {
                subject,
                multiplier,
                title,
            }
        }
    }
}

assert_component!(WeakMatchupHost);
