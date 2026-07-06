mod props;
mod style;

use dioxus::prelude::*;
pub use props::MatchupLabelProps;
use style::CLASS;
use tw_macro::assert_component;
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
