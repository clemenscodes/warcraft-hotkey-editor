mod props;
mod style;

use dioxus::prelude::*;
pub use props::MatchupValueProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(MatchupValue);

/// A matchup cell's value; strong/weak colour comes from the parent cell group.
#[component]
pub fn MatchupValue(props: MatchupValueProps) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}
