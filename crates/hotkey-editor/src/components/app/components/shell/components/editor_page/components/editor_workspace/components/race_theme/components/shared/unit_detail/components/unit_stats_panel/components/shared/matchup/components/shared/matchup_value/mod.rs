mod props;
mod style;

use dioxus::prelude::*;
pub use props::MatchupValueProps;
use style::CLASS;
use tw_macro::assert_component;

/// A matchup cell's value: the damage multiplier rendered as a percentage. The
/// strong/weak colour comes from the parent cell group.
#[component]
pub fn MatchupValue(props: MatchupValueProps) -> Element {
    let percent = props.multiplier * 100.0;
    rsx! {
        span {
            class: CLASS,
            "{percent:.0}%"
        }
    }
}

assert_component!(MatchupValue);
