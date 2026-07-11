mod model;
mod view;

pub use view::MatchupLabelView;
mod style;

use dioxus::prelude::*;
use model::MatchupLabelModel;
use style::CLASS;
use tw_macro::assert_component;

/// A matchup cell's label: the attack or defense type it names, rendered through its
/// own `Display`.
#[component]
pub fn MatchupLabel(props: MatchupLabelModel) -> Element {
    let subject = props.subject;
    rsx! {
        span {
            class: CLASS,
            "{subject}"
        }
    }
}

assert_component!(MatchupLabel);
