mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::MatchupLabelProps;
use style::CLASS;
assert_component!(MatchupLabel);

/// A matchup cell's label.
#[component]
pub fn MatchupLabel(props: MatchupLabelProps) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}
